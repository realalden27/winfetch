#![allow(clippy::collapsible_if)]

use std::collections::HashMap;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt as _;
use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::GetCurrentProcessId;

#[derive(Debug, Clone)]
pub struct ProcessNode {
    pub pid: u32,
    pub parent_pid: u32,
    pub name: String,
}

#[derive(Debug, Clone, Default)]
pub struct ParentContext {
    pub shell: Option<String>,
    pub terminal: Option<String>,
}

/// Helper to sanitize executable names into clean display identifiers.
fn clean_process_name(raw: &str) -> String {
    let lower = raw.to_lowercase();
    let name = lower.strip_suffix(".exe").unwrap_or(&lower);

    match name {
        "pwsh" => "PowerShell 7+".to_string(),
        "powershell" => "Windows PowerShell".to_string(),
        "cmd" => "Command Prompt".to_string(),
        "nu" => "Nushell".to_string(),
        "bash" => "Bash".to_string(),
        "zsh" => "Zsh".to_string(),
        "fish" => "Fish".to_string(),
        "windowsterminal" => "Windows Terminal".to_string(),
        "code" => "VS Code Terminal".to_string(),
        "alacritty" => "Alacritty".to_string(),
        "wezterm" | "wezterm-gui" => "WezTerm".to_string(),
        "conhost" => "Console Host (conhost)".to_string(),
        "mintty" => "Git Bash (MinTTY)".to_string(),
        "hyper" => "Hyper".to_string(),
        "tabby" => "Tabby".to_string(),
        other => other.to_string(),
    }
}

/// Takes an instantaneous Toolhelp32 snapshot to trace parent shell and terminal emulator.
#[must_use]
pub fn detect_shell_and_terminal() -> ParentContext {
    let current_pid = unsafe { GetCurrentProcessId() };

    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot == INVALID_HANDLE_VALUE {
        return ParentContext::default();
    }

    let mut process_map: HashMap<u32, ProcessNode> = HashMap::new();

    unsafe {
        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let null_pos = entry
                    .szExeFile
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szExeFile.len());
                let exe_name = OsString::from_wide(&entry.szExeFile[..null_pos])
                    .into_string()
                    .unwrap_or_default();

                process_map.insert(
                    entry.th32ProcessID,
                    ProcessNode {
                        pid: entry.th32ProcessID,
                        parent_pid: entry.th32ParentProcessID,
                        name: exe_name,
                    },
                );

                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        windows_sys::Win32::Foundation::CloseHandle(snapshot);
    }

    let mut shell: Option<String> = None;
    let mut terminal: Option<String> = None;

    // Traverse upwards from current process PID
    let mut current = current_pid;
    let mut depth: u32 = 0;

    while let Some(node) = process_map.get(&current) {
        if depth > 0 {
            let lower = node.name.to_lowercase();
            let stripped = lower.strip_suffix(".exe").unwrap_or(&lower);

            let is_shell = matches!(stripped, "pwsh" | "powershell" | "cmd" | "nu" | "bash" | "zsh" | "fish");
            let is_terminal = matches!(
                stripped,
                "windowsterminal"
                    | "code"
                    | "alacritty"
                    | "wezterm"
                    | "wezterm-gui"
                    | "conhost"
                    | "mintty"
                    | "hyper"
                    | "tabby"
            );

            if shell.is_none() && is_shell {
                shell = Some(clean_process_name(&node.name));
            } else if terminal.is_none() && is_terminal {
                terminal = Some(clean_process_name(&node.name));
            }

            if shell.is_some() && terminal.is_some() {
                break;
            }
        }

        if node.parent_pid == 0 || node.parent_pid == current {
            break;
        }

        current = node.parent_pid;
        depth = depth.saturating_add(1);
        if depth > 8 {
            break;
        }
    }

    // Fallback: Check environment variables if Toolhelp32 missed anything
    if terminal.is_none() {
        if std::env::var_os("WT_SESSION").is_some() {
            terminal = Some("Windows Terminal".to_string());
        } else if std::env::var_os("TERM_PROGRAM")
            .map(|s| s.to_string_lossy().contains("vscode"))
            .unwrap_or(false)
        {
            terminal = Some("VS Code Terminal".to_string());
        } else if let Ok(term) = std::env::var("TERM") {
            if !term.is_empty() {
                terminal = Some(term);
            }
        }
    }

    ParentContext { shell, terminal }
}
