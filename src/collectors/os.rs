use crate::platform::registry::read_hklm_string;

#[derive(Debug, Clone, serde::Serialize)]
pub struct OsInfo {
    pub display_name: String,
    pub kernel: String,
    pub build: String,
    pub arch: String,
}

/// Gathers comprehensive Windows OS edition, version (24H2, 23H2), and build metadata.
#[must_use]
pub fn collect_os() -> OsInfo {
    let subkey = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";

    let raw_product = read_hklm_string(subkey, "ProductName").unwrap_or_else(|| "Windows".to_string());
    let display_version = read_hklm_string(subkey, "DisplayVersion").or_else(|| read_hklm_string(subkey, "ReleaseId"));
    let current_build = read_hklm_string(subkey, "CurrentBuild")
        .or_else(|| read_hklm_string(subkey, "CurrentBuildNumber"))
        .unwrap_or_else(|| "0".to_string());
    let ubr = read_hklm_string(subkey, "UBR")
        .or_else(|| crate::platform::registry::read_hklm_u32(subkey, "UBR").map(|n| n.to_string()))
        .unwrap_or_else(|| "0".to_string());

    let build_num: u64 = current_build.parse().unwrap_or(0);

    // Microsoft keeps "Windows 10" in the registry ProductName for Windows 11 builds (>= 22000).
    // Correct the display name to Windows 11 if build >= 22000.
    let product_name = if build_num >= 22000 && raw_product.contains("Windows 10") {
        raw_product.replace("Windows 10", "Windows 11")
    } else {
        raw_product
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "aarch64") {
        "ARM64"
    } else {
        "x86"
    };

    let version_tag = display_version
        .map(|v| format!(" ({v} Build {current_build}.{ubr})"))
        .unwrap_or_else(|| format!(" (Build {current_build}.{ubr})"));

    let display_name = format!("{product_name} {arch}{version_tag}");
    let kernel = format!("Windows NT 10.0.{current_build}");

    OsInfo {
        display_name,
        kernel,
        build: format!("{current_build}.{ubr}"),
        arch: arch.to_string(),
    }
}
