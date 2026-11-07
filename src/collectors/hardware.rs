use crate::platform::registry::{read_hklm_string, read_hklm_u32, read_hklm_u64};
use crate::platform::system::get_memory_stats;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HardwareInfo {
    pub host: Option<String>,
    pub cpu: String,
    pub gpus: Vec<String>,
    pub memory: String,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub memory_percentage: u8,
}

/// Formats byte quantities into human-readable GB strings.
fn format_gb(bytes: u64) -> String {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{gb:.2} GB")
}

/// Collects motherboard / system model metadata from the BIOS registry section.
#[must_use]
pub fn collect_host() -> Option<String> {
    let subkey = r"HARDWARE\DESCRIPTION\System\BIOS";

    let sys_mfg = read_hklm_string(subkey, "SystemManufacturer");
    let sys_prod = read_hklm_string(subkey, "SystemProductName");
    let board_mfg = read_hklm_string(subkey, "BaseBoardManufacturer");
    let board_prod = read_hklm_string(subkey, "BaseBoardProduct");

    let is_generic = |s: &str| {
        let l = s.to_lowercase();
        l.contains("to be filled") || l.contains("default string") || l.contains("system manufacturer")
    };

    let host_string = if let (Some(m), Some(p)) = (&board_mfg, &board_prod) {
        if !is_generic(m) && !is_generic(p) {
            if let Some(sp) = &sys_prod {
                if !is_generic(sp) && sp != p {
                    format!("{m} {p} ({sp})")
                } else {
                    format!("{m} {p}")
                }
            } else {
                format!("{m} {p}")
            }
        } else {
            sys_prod
                .filter(|p| !is_generic(p))
                .unwrap_or_else(|| "PC System".to_string())
        }
    } else if let (Some(m), Some(p)) = (&sys_mfg, &sys_prod) {
        if !is_generic(m) && !is_generic(p) {
            format!("{m} {p}")
        } else {
            "PC System".to_string()
        }
    } else {
        sys_prod
            .filter(|p| !is_generic(p))
            .unwrap_or_else(|| "PC System".to_string())
    };

    if host_string.trim().is_empty() || is_generic(&host_string) {
        None
    } else {
        Some(host_string)
    }
}

/// Collects CPU brand string, thread count, and base/advertised clock frequency.
#[must_use]
pub fn collect_cpu() -> String {
    let subkey = r"HARDWARE\DESCRIPTION\System\CentralProcessor\0";

    let raw_name = read_hklm_string(subkey, "ProcessorNameString").unwrap_or_else(|| "Unknown CPU".to_string());
    let cleaned_name = raw_name.split_whitespace().collect::<Vec<&str>>().join(" ");

    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);

    let mhz = read_hklm_u32(subkey, "~MHz");
    let clock_str = mhz
        .map(|m| {
            let ghz = m as f64 / 1000.0;
            format!(" @ {ghz:.2} GHz")
        })
        .unwrap_or_default();

    format!("{cleaned_name} ({threads}){clock_str}")
}

/// Collects all discrete and integrated GPU display adapters with dedicated VRAM.
#[must_use]
pub fn collect_gpus() -> Vec<String> {
    let base_class = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}";
    let mut gpus = Vec::new();

    // Iterate through hardware display subkeys (0000, 0001, 0002, ...)
    for i in 0..16 {
        let subkey = format!("{base_class}\\{i:04}");

        if let Some(desc) = read_hklm_string(&subkey, "DriverDesc") {
            let lower = desc.to_lowercase();
            // Filter out remote / virtual display drivers
            if lower.contains("mirage")
                || lower.contains("rdp")
                || lower.contains("citrix")
                || lower.contains("parsec")
                || lower.contains("virtual")
            {
                continue;
            }

            // Read dedicated video memory size if present
            let mem_bytes = read_hklm_u64(&subkey, "HardwareInformation.qwMemorySize")
                .or_else(|| read_hklm_u64(&subkey, "qwMemorySize"))
                .or_else(|| {
                    read_hklm_u32(&subkey, "HardwareInformation.MemorySize")
                        .or_else(|| read_hklm_u32(&subkey, "MemorySize"))
                        .map(|m| m as u64)
                });

            let vram_str = mem_bytes
                .filter(|&b| b > 0)
                .map(|b| {
                    let gb = b as f64 / (1024.0 * 1024.0 * 1024.0);
                    if gb >= 1.0 {
                        format!(" ({:.0} GB)", gb.round())
                    } else {
                        let mb = b / (1024 * 1024);
                        format!(" ({mb} MB)")
                    }
                })
                .unwrap_or_default();

            let entry = format!("{desc}{vram_str}");
            if !gpus.contains(&entry) {
                gpus.push(entry);
            }
        }
    }

    if gpus.is_empty() {
        vec!["Standard Display Adapter".to_string()]
    } else {
        gpus
    }
}

/// Collects RAM statistics including used, total, and percentage.
#[must_use]
pub fn collect_memory() -> (String, u64, u64, u8) {
    if let Some(stats) = get_memory_stats() {
        let pct = if stats.total_bytes > 0 {
            ((stats.used_bytes as f64 / stats.total_bytes as f64) * 100.0).round() as u8
        } else {
            0
        };

        let formatted = format!(
            "{} / {} ({}%)",
            format_gb(stats.used_bytes),
            format_gb(stats.total_bytes),
            pct.min(100)
        );

        (formatted, stats.used_bytes, stats.total_bytes, pct.min(100))
    } else {
        ("Unavailable".to_string(), 0, 0, 0)
    }
}
