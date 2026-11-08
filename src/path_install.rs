#![allow(clippy::collapsible_if)]

use std::path::{Path, PathBuf};

/// Notification indicating whether `winfetch` was newly installed or upgraded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallNotice {
    FirstRun(PathBuf),
    Upgraded {
        from: (u64, u64, u64),
        to: String,
        target: PathBuf,
    },
}

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

fn copy_to_windows_apps(src: &Path) {
    #[cfg(target_os = "windows")]
    {
        if let Some(local_appdata) = std::env::var_os("LOCALAPPDATA") {
            let winapps_exe = PathBuf::from(local_appdata)
                .join("Microsoft")
                .join("WindowsApps")
                .join("winfetch.exe");
            let _ = std::fs::copy(src, winapps_exe);
        }
    }
}

/// Ensures `winfetch` is copied into the OS user programs directory and added to user PATH.
#[must_use]
pub fn ensure_in_path() -> Option<InstallNotice> {
    if cfg!(test) {
        return None;
    }

    let Ok(current_exe) = std::env::current_exe() else {
        return None;
    };

    if is_dev_or_temp_binary(&current_exe) {
        return None;
    }

    let target_exe = get_user_programs_exe()?;

    let Ok(canonical_current) = current_exe.canonicalize() else {
        return None;
    };

    if let Ok(canonical_target) = target_exe.canonicalize() {
        if canonical_current == canonical_target {
            return None;
        }
    }

    let current_version_str = env!("CARGO_PKG_VERSION");
    let current_version = parse_version(current_version_str).unwrap_or((0, 0, 0));

    if target_exe.exists() {
        if let Some(installed_version) = read_binary_version(&target_exe) {
            if current_version <= installed_version {
                return None;
            }

            // Upgrade older installed binary
            if let Some(parent) = target_exe.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if std::fs::copy(&current_exe, &target_exe).is_ok() {
                copy_to_windows_apps(&current_exe);
                return Some(InstallNotice::Upgraded {
                    from: installed_version,
                    to: current_version_str.to_string(),
                    target: target_exe,
                });
            }
            return None;
        }
    }

    // First-run installation
    let target_dir = target_exe.parent()?;

    if std::fs::create_dir_all(target_dir).is_err() {
        return None;
    }

    if std::fs::copy(&current_exe, &target_exe).is_err() {
        return None;
    }

    copy_to_windows_apps(&current_exe);

    #[cfg(target_os = "windows")]
    {
        let target_dir_str = target_dir.to_string_lossy();
        register_windows_user_path(&target_dir_str);
    }

    Some(InstallNotice::FirstRun(target_exe))
}

#[cfg(target_os = "windows")]
fn register_windows_user_path(dir_to_add: &str) {
    let script = format!(
        r"$p = [Environment]::GetEnvironmentVariable('Path', 'User'); if ($p -notlike '*{dir_to_add}*') {{ [Environment]::SetEnvironmentVariable('Path', ($p.TrimEnd(';') + ';{dir_to_add}'), 'User') }}"
    );

    let _ = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output();
}
