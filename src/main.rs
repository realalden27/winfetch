use std::collections::HashSet;
use std::time::Instant;

use clap::Parser as _;
use winfetch::art::get_logo;
use winfetch::collectors::collect_all_parallel;
use winfetch::config::{Cli, Commands};
use winfetch::ensure_in_path;
use winfetch::ui::{FormatOptions, format_fetch};
use winfetch::update::check_and_update;

#[cfg(windows)]
fn enable_vt_mode() {
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        STD_OUTPUT_HANDLE,
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if !handle.is_null() && handle != -1isize as *mut _ {
            let mut mode = 0u32;
            if GetConsoleMode(handle, &mut mode) != 0 {
                let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
}

#[cfg(not(windows))]
fn enable_vt_mode() {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_vt_mode();
    ensure_in_path();

    let cli = Cli::parse();

    if let Some(Commands::Update) = cli.command {
        if let Err(e) = check_and_update().await {
            eprintln!("Update error: {e}");
            std::process::exit(1);
        }
        return Ok(());
    }

    let start_time = Instant::now();

    // Parallel non-blocking execution across thread pool
    let info = collect_all_parallel();

    if cli.json {
        let json_str = serde_json::to_string_pretty(&info)?;
        println!("{json_str}");
    } else {
        let is_win11 = info.os.display_name.contains("Windows 11")
            || info
                .os
                .build
                .split('.')
                .next()
                .unwrap_or("0")
                .parse::<u64>()
                .unwrap_or(0)
                >= 22000;

        let chosen_logo = if cli.minimal && cli.logo == winfetch::art::LogoStyle::Auto {
            winfetch::art::LogoStyle::Small
        } else {
            cli.logo
        };

        let logo = get_logo(chosen_logo, is_win11);

        let fields_filter = cli.only.map(|s| {
            s.split(',')
                .map(|f| f.trim().to_lowercase())
                .collect::<HashSet<String>>()
        });

        let format_opts = FormatOptions {
            color: cli.color,
            minimal: cli.minimal,
            fields_filter,
        };

        let rendered = format_fetch(&info, &logo, &format_opts);
        print!("{rendered}");
    }

    if cli.time {
        let elapsed = start_time.elapsed();
        let ms = elapsed.as_secs_f64() * 1000.0;
        println!("\nRendered in {ms:.2}ms");
    }

    Ok(())
}
