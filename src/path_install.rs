#![allow(clippy::collapsible_if)]

use std::path::{Path, PathBuf};

/// Parses a semver version string into (major, minor, patch).
#[must_use]
pub fn parse_version(s: &str) -> Option<(u64, u64, u64)> {
    let clean = s.trim();
    let clean = clean.strip_prefix("winfetch").unwrap_or(clean).trim();
    let clean = clean.trim_start_matches('v').trim();

    let parts: Vec<&str> = clean.split('.').collect();
    let major = parts.first()?.parse::<u64>().ok()?;
    let minor = parts.get(1)?.parse::<u64>().ok()?;
    let patch_str = parts.get(2)?;
    let patch = patch_str.split('-').next()?.parse::<u64>().ok()?;
    Some((major, minor, patch))
}

/// Identifies whether the given binary path is running from a local Cargo development build.
#[must_use]
pub fn is_dev_or_temp_binary(path: &Path) -> bool {
    let path_str = path.to_string_lossy().replace('\\', "/").to_lowercase();
    path_str.contains("/target/debug/")
        || path_str.contains("/target/release/")
        || path_str.contains("/target/package/")
        || path.starts_with(std::env::temp_dir())
}

/// Computes the standard OS user programs binary path.
#[must_use]
pub fn get_user_programs_exe() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let local_appdata = std::env::var_os("LOCALAPPDATA")?;
        Some(
            PathBuf::from(local_appdata)
                .join("Programs")
                .join("winfetch")
                .join("winfetch.exe"),
        )
    }
    #[cfg(not(target_os = "windows"))]
    {
        let home = std::env::var_os("HOME")?;
        Some(PathBuf::from(home).join(".local").join("bin").join("winfetch"))
    }
}

/// Reads the current version of an installed executable.
fn read_binary_version(path: &Path) -> Option<(u64, u64, u64)> {
    let output = std::process::Command::new(path).arg("--version").output().ok()?;
    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        parse_version(&text)
    } else {
        None
    }
}

/// Ensures `winfetch` is copied into the OS user programs directory and added to user PATH.
pub fn ensure_in_path() {
    if cfg!(test) {
        return;
    }

    let Ok(current_exe) = std::env::current_exe() else {
        return;
    };

    if is_dev_or_temp_binary(&current_exe) {
        return;
    }

    let Some(target_exe) = get_user_programs_exe() else {
        return;
    };

    let Ok(canonical_current) = current_exe.canonicalize() else {
        return;
    };

    if let Ok(canonical_target) = target_exe.canonicalize() {
        if canonical_current == canonical_target {
            return;
        }
    }

    let current_version_str = env!("CARGO_PKG_VERSION");
    let current_version = parse_version(current_version_str).unwrap_or((0, 0, 0));

    if target_exe.exists() {
        if let Some(installed_version) = read_binary_version(&target_exe) {
            if current_version <= installed_version {
                return;
            }

            // Upgrade older installed binary
            if let Some(parent) = target_exe.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if std::fs::copy(&current_exe, &target_exe).is_ok() {
                println!(
                    "winfetch has been upgraded from v{}.{}.{} to v{} in:\n  {}",
                    installed_version.0,
                    installed_version.1,
                    installed_version.2,
                    current_version_str,
                    target_exe.display()
                );
            }
            return;
        }
    }

    // First-run installation
    let Some(target_dir) = target_exe.parent() else {
        return;
    };

    if std::fs::create_dir_all(target_dir).is_err() {
        return;
    }

    if std::fs::copy(&current_exe, &target_exe).is_err() {
        return;
    }

    #[cfg(target_os = "windows")]
    {
        let target_dir_str = target_dir.to_string_lossy();
        register_windows_user_path(&target_dir_str);
    }

    println!(
        "winfetch has been installed to your user programs directory:\n  {}\nAdded to your User PATH. You can now run 'winfetch' from any terminal window!",
        target_exe.display()
    );
}

#[cfg(target_os = "windows")]
fn register_windows_user_path(dir_to_add: &str) {
    let script = format!(
        r"[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';{dir_to_add}', 'User')"
    );

    let _ = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output();
}
