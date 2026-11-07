#![allow(clippy::collapsible_if)]

use std::collections::HashSet;

use crate::art::Logo;
use crate::collectors::SystemInfo;
use crate::ui::colors::{AccentColor, C_BLUE, C_CYAN, C_GREEN, C_MAGENTA, C_RED, C_YELLOW, DIM, RESET};

#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub color: AccentColor,
    pub minimal: bool,
    pub fields_filter: Option<HashSet<String>>,
}

/// Strip ANSI escape codes to calculate visual character length accurately.
#[must_use]
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Renders the 16-color ANSI terminal palette as two distinct rows.
#[must_use]
pub fn render_ansi_palette() -> (String, String) {
    let mut row1 = String::new();
    for col in 40..=47 {
        row1.push_str(&format!("\x1b[{col}m   {RESET}"));
    }
    let mut row2 = String::new();
    for col in 100..=107 {
        row2.push_str(&format!("\x1b[{col}m   {RESET}"));
    }
    (row1, row2)
}

fn format_user_host(user_host: &str, accent: &str, dim: &str, reset: &str) -> String {
    if let Some((user, host)) = user_host.split_once('@') {
        format!("{accent}{user}{reset}{dim}@{reset}{accent}{host}{reset}")
    } else {
        format!("{accent}{user_host}{reset}")
    }
}

fn colorize_disk_bar(formatted: &str, accent: &str) -> String {
    if let Some((info_part, bar_part)) = formatted.split_once('[') {
        let bar_inner = bar_part.trim_end_matches(']');
        let filled_count = bar_inner.chars().take_while(|&c| c == '=').count();
        let empty_count = bar_inner.len().saturating_sub(filled_count);
        let filled: String = "=".repeat(filled_count);
        let empty: String = " ".repeat(empty_count);
        format!("{info_part}{DIM}[{RESET}{accent}{filled}{RESET}{DIM}{empty}]{RESET}")
    } else {
        formatted.to_string()
    }
}

#[must_use]
pub fn format_fetch(info: &SystemInfo, logo: &Logo, opts: &FormatOptions) -> String {
    let reset = RESET;
    let dim = DIM;

    let get_color = |default_color: &'static str| -> String {
        if opts.color == AccentColor::Multi {
            default_color.to_string()
        } else {
            opts.color.bold_code()
        }
    };

    let c_red = get_color(C_RED);
    let c_grn = get_color(C_GREEN);
    let c_yel = get_color(C_YELLOW);
    let c_blu = get_color(C_BLUE);
    let c_mag = get_color(C_MAGENTA);
    let c_cyn = get_color(C_CYAN);

    // CPU brand-aware accent
    let c_cpu = if opts.color == AccentColor::Multi {
        let cpu_upper = info.cpu.to_uppercase();
        if cpu_upper.contains("AMD") || cpu_upper.contains("RYZEN") {
            C_RED.to_string()
        } else if cpu_upper.contains("INTEL") {
            C_BLUE.to_string()
        } else {
            C_GREEN.to_string()
        }
    } else {
        opts.color.bold_code()
    };

    // GPU brand-aware accent
    let get_gpu_color = |gpu_name: &str| -> String {
        if opts.color == AccentColor::Multi {
            let gpu_upper = gpu_name.to_uppercase();
            if gpu_upper.contains("NVIDIA")
                || gpu_upper.contains("GEFORCE")
                || gpu_upper.contains("RTX")
                || gpu_upper.contains("GTX")
            {
                C_GREEN.to_string()
            } else if gpu_upper.contains("AMD") || gpu_upper.contains("RADEON") {
                C_RED.to_string()
            } else if gpu_upper.contains("INTEL") || gpu_upper.contains("ARC") {
                C_BLUE.to_string()
            } else {
                C_MAGENTA.to_string()
            }
        } else {
            opts.color.bold_code()
        }
    };

    let should_include = |field: &str| -> bool {
        if let Some(set) = &opts.fields_filter {
            set.contains(field)
        } else {
            true
        }
    };

    let mut spec_lines: Vec<String> = Vec::new();

    if opts.minimal {
        let user_host = format_user_host(&info.environment.user_at_host, &c_cyn, dim, reset);
        spec_lines.push(user_host);
        spec_lines.push(format!("{c_red}OS:{reset} {}", info.os.display_name));
        spec_lines.push(format!("{c_cpu}CPU:{reset} {}", info.cpu));
        spec_lines.push(format!("{c_yel}Memory:{reset} {}", info.memory));
        spec_lines.push(format!("{c_grn}Uptime:{reset} {}", info.environment.uptime));
    } else {
        // 1. User @ Host header & underline
        let user_host = format_user_host(&info.environment.user_at_host, &c_cyn, dim, reset);
        let plain_len = strip_ansi(&user_host).chars().count();
        spec_lines.push(user_host);
        spec_lines.push(format!("{dim}{}{reset}", "-".repeat(plain_len)));

        // 2. OS
        if should_include("os") {
            spec_lines.push(format!("{c_red}OS:{reset} {}", info.os.display_name));
        }

        // 3. Host / Motherboard
        if should_include("host") {
            if let Some(host) = &info.host {
                spec_lines.push(format!("{c_yel}Host:{reset} {host}"));
            }
        }

        // 4. Kernel
        if should_include("kernel") {
            spec_lines.push(format!("{c_blu}Kernel:{reset} {}", info.os.kernel));
        }

        // 5. Uptime
        if should_include("uptime") {
            spec_lines.push(format!("{c_grn}Uptime:{reset} {}", info.environment.uptime));
        }

        // 6. Packages
        if should_include("packages") {
            if let Some(pkg_str) = info.packages.to_display_string() {
                spec_lines.push(format!("{c_mag}Packages:{reset} {pkg_str}"));
            }
        }

        // 7. Shell
        if should_include("shell") {
            if let Some(shell) = &info.environment.shell {
                spec_lines.push(format!("{c_cyn}Shell:{reset} {shell}"));
            }
        }

        // 8. Terminal
        if should_include("terminal") {
            if let Some(terminal) = &info.environment.terminal {
                spec_lines.push(format!("{c_blu}Terminal:{reset} {terminal}"));
            }
        }

        // 9. Display & Refresh Rate
        if should_include("display") {
            if let Some(display) = &info.environment.display {
                spec_lines.push(format!("{c_yel}Display:{reset} {display}"));
            }
        }

        // 10. CPU
        if should_include("cpu") {
            spec_lines.push(format!("{c_cpu}CPU:{reset} {}", info.cpu));
        }

        // 11. GPUs
        if should_include("gpu") {
            for (idx, gpu) in info.gpus.iter().enumerate() {
                let color = get_gpu_color(gpu);
                let label = if idx == 0 { "GPU:" } else { "    " };
                spec_lines.push(format!("{color}{label}{reset} {gpu}"));
            }
        }

        // 12. Memory
        if should_include("memory") || should_include("ram") {
            spec_lines.push(format!("{c_yel}Memory:{reset} {}", info.memory));
        }

        // 13. Disks
        if should_include("disk") || should_include("storage") {
            for disk in &info.disks {
                let colored_formatted = colorize_disk_bar(&disk.formatted, &c_cyn);
                spec_lines.push(format!("{c_cyn}Disk ({}):{reset} {colored_formatted}", disk.mount));
            }
        }

        // 14. Battery
        if should_include("battery") {
            if let Some(bat) = &info.environment.battery {
                spec_lines.push(format!("{c_grn}Battery:{reset} {bat}"));
            }
        }

        // 15. ANSI Color Palette
        spec_lines.push(String::new());
        let (pal1, pal2) = render_ansi_palette();
        spec_lines.push(pal1);
        spec_lines.push(pal2);
    }

    // Two-column side-by-side composition
    if logo.lines.is_empty() {
        return spec_lines.join("\n");
    }

    let mut output = String::new();
    let top_offset = if spec_lines.len() > logo.lines.len() + 2 {
        2
    } else if spec_lines.len() > logo.lines.len() {
        1
    } else {
        0
    };
    let total_rows = std::cmp::max(logo.lines.len() + top_offset, spec_lines.len());
    let logo_col_width = logo.width;
    let separator = "   ";

    for row in 0..total_rows {
        // Left Column (Logo)
        let in_logo_range = row >= top_offset && (row - top_offset) < logo.lines.len();
        if in_logo_range {
            let logo_idx = row - top_offset;
            let logo_line = logo.lines.get(logo_idx).map(|s| s.as_str()).unwrap_or("");
            let visual_logo_len = strip_ansi(logo_line).chars().count();
            let padding = " ".repeat(logo_col_width.saturating_sub(visual_logo_len));

            output.push_str(logo_line);
            output.push_str(&padding);
        } else {
            let padding = " ".repeat(logo_col_width);
            output.push_str(&padding);
        }
        output.push_str(separator);

        // Right Column (Spec)
        if let Some(spec_line) = spec_lines.get(row) {
            output.push_str(spec_line);
        }

        output.push('\n');
    }

    // Clean up trailing empty lines
    while output.ends_with("\n\n") {
        output.pop();
    }

    output
}
