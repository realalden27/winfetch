use crate::platform::drives::{DriveInfo, get_fixed_drives};

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiskDisplay {
    pub mount: String,
    pub formatted: String,
    pub percentage: u8,
}

/// Formats raw byte count into readable GB.
fn format_gb(bytes: u64) -> String {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{gb:.0} GB")
}

/// Generates a compact ASCII progress bar (e.g. `[=====>     ]`).
#[must_use]
pub fn render_progress_bar(percentage: u8, width: usize) -> String {
    let clamped = (percentage as usize).min(100);
    let filled = (clamped * width) / 100;
    let empty = width.saturating_sub(filled);

    let mut bar = String::with_capacity(width + 2);
    bar.push('[');
    for _ in 0..filled {
        bar.push('=');
    }
    for _ in 0..empty {
        bar.push(' ');
    }
    bar.push(']');
    bar
}

/// Collects and formats all fixed local disk partitions.
#[must_use]
pub fn collect_disks() -> (Vec<DiskDisplay>, Vec<DriveInfo>) {
    let raw_drives = get_fixed_drives();
    let mut displays = Vec::new();

    for d in &raw_drives {
        let used_str = format_gb(d.used_bytes);
        let total_str = format_gb(d.total_bytes);
        let bar = render_progress_bar(d.used_percentage, 10);

        let formatted = format!("{used_str} / {total_str} ({}%) {bar}", d.used_percentage);

        displays.push(DiskDisplay {
            mount: d.mount.clone(),
            formatted,
            percentage: d.used_percentage,
        });
    }

    (displays, raw_drives)
}
