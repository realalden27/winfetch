use crate::platform::process::detect_shell_and_terminal;
use crate::platform::system::{get_battery_info, get_display_info, get_uptime_seconds};

#[derive(Debug, Clone, serde::Serialize)]
pub struct EnvironmentInfo {
    pub user_at_host: String,
    pub uptime: String,
    pub shell: Option<String>,
    pub terminal: Option<String>,
    pub display: Option<String>,
    pub battery: Option<String>,
}

/// Formats raw uptime in seconds into clean human-readable units.
#[must_use]
pub fn format_uptime(total_seconds: u64) -> String {
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if days > 0 {
        format!("{days} days, {hours} hours, {minutes} mins")
    } else if hours > 0 {
        format!("{hours} hours, {minutes} mins")
    } else {
        format!("{minutes} mins")
    }
}

/// Collects environment, uptime, user/host, display, shell, and battery details.
#[must_use]
pub fn collect_environment() -> EnvironmentInfo {
    let user = std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "user".to_string());
    let host = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "windows".to_string());
    let user_at_host = format!("{user}@{host}");

    let uptime = format_uptime(get_uptime_seconds());

    let proc_ctx = detect_shell_and_terminal();

    let display = get_display_info().map(|d| {
        if d.refresh_rate_hz > 0 {
            format!("{}x{} @ {}Hz", d.width, d.height, d.refresh_rate_hz)
        } else {
            format!("{}x{}", d.width, d.height)
        }
    });

    let battery = get_battery_info().map(|b| {
        let status = if b.is_plugged_in { "Charging" } else { "Discharging" };
        format!("{}% ({status})", b.percentage)
    });

    EnvironmentInfo {
        user_at_host,
        uptime,
        shell: proc_ctx.shell,
        terminal: proc_ctx.terminal,
        display,
        battery,
    }
}
