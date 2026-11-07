use std::io::Cursor;

#[derive(serde::Deserialize, Debug)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(serde::Deserialize, Debug)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(serde::Deserialize, Debug)]
struct CratesResponse {
    #[serde(rename = "crate")]
    krate: Option<CrateMeta>,
}

#[derive(serde::Deserialize, Debug)]
struct CrateMeta {
    max_version: Option<String>,
}

fn is_cargo_install(exe: &std::path::Path) -> bool {
    exe.components()
        .any(|c| c.as_os_str().to_string_lossy().contains(".cargo"))
}

#[must_use]
pub fn get_target_triple() -> &'static str {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        "x86_64-pc-windows-msvc"
    }
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        "aarch64-pc-windows-msvc"
    }
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    {
        "i686-pc-windows-msvc"
    }
    #[cfg(not(target_os = "windows"))]
    {
        "unknown"
    }
}

/// Checks crates.io and GitHub Releases and updates the running binary in-place.
pub async fn check_and_update() -> Result<(), Box<dyn std::error::Error>> {
    let current_version_str = env!("CARGO_PKG_VERSION");
    let current_version = crate::path_install::parse_version(current_version_str).unwrap_or((0, 0, 0));

    println!("Checking for updates on crates.io and GitHub...");

    let client = reqwest::Client::builder()
        .user_agent(format!(
            "winfetch/{current_version_str} (https://github.com/realalden27/winfetch)"
        ))
        .build()?;

    // 1. Query crates.io API
    let crates_url = "https://crates.io/api/v1/crates/winfetch";
    let crates_version = match client.get(crates_url).send().await {
        Ok(resp) if resp.status().is_success() => {
            if let Ok(data) = resp.json::<CratesResponse>().await {
                data.krate.and_then(|k| k.max_version)
            } else {
                None
            }
        },
        _ => None,
    };

    // 2. Query GitHub Releases API
    let github_url = "https://api.github.com/repos/realalden27/winfetch/releases/latest";
    let release: Option<GitHubRelease> = match client.get(github_url).send().await {
        Ok(resp) if resp.status().is_success() => resp.json::<GitHubRelease>().await.ok(),
        _ => None,
    };

    let latest_github_version_str = release
        .as_ref()
        .map(|r| r.tag_name.trim().trim_start_matches('v').to_string());

    let github_parsed = latest_github_version_str
        .as_deref()
        .and_then(crate::path_install::parse_version)
        .unwrap_or((0, 0, 0));
    let crates_parsed = crates_version
        .as_deref()
        .and_then(crate::path_install::parse_version)
        .unwrap_or((0, 0, 0));

    let max_available = std::cmp::max(github_parsed, crates_parsed);

    if max_available <= current_version {
        println!("winfetch is already up to date (v{current_version_str}).");
        return Ok(());
    }

    let target_version_str = if crates_parsed >= github_parsed {
        crates_version.as_deref().unwrap_or(current_version_str)
    } else {
        latest_github_version_str.as_deref().unwrap_or(current_version_str)
    };

    println!("Found newer version v{target_version_str} (current: v{current_version_str}).");

    // If currently running from a cargo installation (.cargo/bin), update via cargo install
    if std::env::current_exe().is_ok_and(|p| is_cargo_install(&p)) {
        println!("Updating via cargo install winfetch...");
        let status = std::process::Command::new("cargo")
            .args(["install", "winfetch"])
            .status();
        if matches!(status, Ok(s) if s.success()) {
            println!("Successfully updated winfetch via cargo to v{target_version_str}!");
            return Ok(());
        }
        println!("cargo install failed or cargo not found; attempting GitHub binary download...");
    }

    let release = match release {
        Some(r) => r,
        None => {
            return Err(
                "New version detected on crates.io, but GitHub Releases could not be reached. Run 'cargo install winfetch' to update."
                    .into(),
            );
        },
    };

    let target = get_target_triple();
    let matching_asset = release.assets.iter().find(|a| a.name.contains(target));

    let asset = match matching_asset {
        Some(a) => a,
        None => {
            return Err(format!(
                "No release binary found for target '{target}' in release '{}'. Please install manually from {}",
                release.tag_name, "https://github.com/realalden27/winfetch/releases"
            )
            .into());
        },
    };

    println!(
        "Found newer version {} (current: v{}). Downloading {}...",
        release.tag_name, current_version_str, asset.name
    );

    let bytes = client.get(&asset.browser_download_url).send().await?.bytes().await?;

    let temp_dir = std::env::temp_dir();
    let temp_exe = temp_dir.join(format!(
        "winfetch_update_{}.tmp",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    ));

    if asset.name.ends_with(".zip") {
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
        let mut found = false;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name().replace('\\', "/");
            if name.ends_with("winfetch.exe") || name.ends_with("winfetch") {
                let mut outfile = std::fs::File::create(&temp_exe)?;
                std::io::copy(&mut file, &mut outfile)?;
                found = true;
                break;
            }
        }
        if !found {
            return Err("Executable not found inside downloaded archive".into());
        }
    } else {
        std::fs::write(&temp_exe, &bytes)?;
    }

    self_replace::self_replace(&temp_exe)?;
    let _ = std::fs::remove_file(&temp_exe);

    println!("Successfully updated winfetch to {}!", release.tag_name);

    Ok(())
}
