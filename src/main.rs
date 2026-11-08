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
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetStdHandle, STD_OUTPUT_HANDLE, SetConsoleMode,
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

fn pause_if_direct_launch(force_pause: bool) {
    let ctx = winfetch::platform::process::detect_shell_and_terminal();
    if force_pause || ctx.shell.is_none() {
        println!("\nPress Enter to exit...");
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_vt_mode();
    let install_notice = ensure_in_path();

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

    if cli.post_install {
        println!("\n\x1b[1;36m--------------------------------------------------------------------------------\x1b[0m");
        println!(" \x1b[1mwinfetch has been installed successfully!\x1b[0m");
        println!(
            " You can now run '\x1b[1;32mwinfetch\x1b[0m' from any terminal (PowerShell, Command Prompt, or Windows Terminal)."
        );
        println!("\x1b[1;36m--------------------------------------------------------------------------------\x1b[0m");
        pause_if_direct_launch(true);
    } else if let Some(notice) = install_notice {
        match notice {
            winfetch::InstallNotice::FirstRun(path) => {
                println!(
                    "\n\x1b[1;36m--------------------------------------------------------------------------------\x1b[0m"
                );
                println!(" \x1b[1mwinfetch has been installed to your User PATH:\x1b[0m");
                println!("   {}", path.display());
                println!();
                println!(
                    " You can now run '\x1b[1;32mwinfetch\x1b[0m' from any terminal (PowerShell, Command Prompt, or Windows Terminal)!"
                );
                println!(
                    "\x1b[1;36m--------------------------------------------------------------------------------\x1b[0m"
                );
            },
            winfetch::InstallNotice::Upgraded { from, to, target } => {
                println!(
                    "\n\x1b[1;36m--------------------------------------------------------------------------------\x1b[0m"
                );
                println!(
                    " winfetch has been upgraded from v{}.{}.{} to v{} in:",
                    from.0, from.1, from.2, to
                );
                println!("   {}", target.display());
                println!(
                    "\x1b[1;36m--------------------------------------------------------------------------------\x1b[0m"
                );
            },
        }
        pause_if_direct_launch(false);
    } else {
        pause_if_direct_launch(false);
    }

    Ok(())
}
