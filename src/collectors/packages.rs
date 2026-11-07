#![allow(clippy::collapsible_if)]

use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct PackageCounts {
    pub winget: Option<usize>,
    pub scoop: Option<usize>,
    pub cargo: Option<usize>,
    pub choco: Option<usize>,
    pub npm: Option<usize>,
}

impl PackageCounts {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.winget.is_none()
            && self.scoop.is_none()
            && self.cargo.is_none()
            && self.choco.is_none()
            && self.npm.is_none()
    }

    #[must_use]
    pub fn to_display_string(&self) -> Option<String> {
        let mut parts = Vec::new();

        if let Some(c) = self.winget {
            if c > 0 {
                parts.push(format!("{c} (winget)"));
            }
        }
        if let Some(c) = self.cargo {
            if c > 0 {
                parts.push(format!("{c} (cargo)"));
            }
        }
        if let Some(c) = self.scoop {
            if c > 0 {
                parts.push(format!("{c} (scoop)"));
            }
        }
        if let Some(c) = self.choco {
            if c > 0 {
                parts.push(format!("{c} (choco)"));
            }
        }
        if let Some(c) = self.npm {
            if c > 0 {
                parts.push(format!("{c} (npm)"));
            }
        }

        if parts.is_empty() { None } else { Some(parts.join(", ")) }
    }
}

/// Helper to count immediate directories inside a directory.
fn count_subdirs(path: &PathBuf, exclude: Option<&str>) -> Option<usize> {
    let entries = std::fs::read_dir(path).ok()?;
    let mut count: usize = 0;
    for entry in entries.flatten() {
        if let Ok(ft) = entry.file_type() {
            if ft.is_dir() {
                if let Some(ex) = exclude {
                    if entry.file_name() == ex {
                        continue;
                    }
                }
                count = count.saturating_add(1);
            }
        }
    }
    if count > 0 { Some(count) } else { None }
}

/// Helper to count files or links in a directory.
fn count_files(path: &PathBuf) -> Option<usize> {
    let entries = std::fs::read_dir(path).ok()?;
    let mut count: usize = 0;
    for entry in entries.flatten() {
        if let Ok(ft) = entry.file_type() {
            if ft.is_file() || ft.is_symlink() {
                count = count.saturating_add(1);
            }
        }
    }
    if count > 0 { Some(count) } else { None }
}

/// High-speed directory inspections to count installed packages across package managers.
#[must_use]
pub fn collect_packages() -> PackageCounts {
    let mut counts = PackageCounts::default();

    // 1. Winget (count links in %LOCALAPPDATA%\Microsoft\WinGet\Links)
    if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
        let winget_path = PathBuf::from(local_appdata).join(r"Microsoft\WinGet\Links");
        counts.winget = count_files(&winget_path);
    }

    // 2. Scoop (count subdirs in %USERPROFILE%\scoop\apps, excluding 'scoop')
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        let scoop_path = PathBuf::from(&user_profile).join(r"scoop\apps");
        counts.scoop = count_subdirs(&scoop_path, Some("scoop"));
    }

    // 3. Cargo (count packages in %USERPROFILE%\.cargo\.crates.toml or bin count)
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        let crates_toml = PathBuf::from(&user_profile).join(r".cargo\.crates.toml");
        if let Ok(content) = std::fs::read_to_string(&crates_toml) {
            let mut cargo_count: usize = 0;
            let mut in_v1 = false;
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed == "[v1]" {
                    in_v1 = true;
                    continue;
                }
                if in_v1 && trimmed.starts_with('"') {
                    cargo_count = cargo_count.saturating_add(1);
                }
            }
            if cargo_count > 0 {
                counts.cargo = Some(cargo_count);
            }
        } else {
            let cargo_bin = PathBuf::from(&user_profile).join(r".cargo\bin");
            counts.cargo = count_files(&cargo_bin);
        }
    }

    // 4. Chocolatey (count subdirs in C:\ProgramData\chocolatey\lib)
    let choco_path = PathBuf::from(r"C:\ProgramData\chocolatey\lib");
    counts.choco = count_subdirs(&choco_path, Some("chocolatey"));

    // 5. Global npm packages
    if let Ok(appdata) = std::env::var("APPDATA") {
        let npm_path = PathBuf::from(appdata).join(r"npm\node_modules");
        counts.npm = count_subdirs(&npm_path, None);
    }

    counts
}
