# winfetch

[![Crates.io](https://img.shields.io/crates/v/winfetch.svg)](https://crates.io/crates/winfetch)
[![Docs.rs](https://docs.rs/winfetch/badge.svg)](https://docs.rs/winfetch)
[![CI](https://github.com/realalden27/winfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/realalden27/winfetch/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

A lightweight, zero-WMI system information tool for Windows written in Rust.

`winfetch` queries system specifications directly through native Win32 registry and kernel APIs, avoiding the overhead of WMI (Windows Management Instrumentation) and PowerShell startup costs. It features multi-threaded collection, accurate Windows 11 build detection, display refresh rate reporting, and clean ASCII logos.

---

## Features

- **Zero-WMI Architecture**: Queries hardware, OS details, and memory metrics directly via Win32 APIs and the Windows registry.
- **Parallel Collectors**: Hardware, storage, and package counts are gathered concurrently across threads via Rayon.
- **Accurate OS & Build Detection**: Accurately identifies Windows 11 editions and major updates (e.g., 24H2, 23H2) by verifying kernel build numbers.
- **Hardware & Display Details**: Reports CPU core topology, motherboard models, GPU adapter names with dedicated VRAM, and primary screen resolution with refresh rate (Hz).
- **Process Hierarchy Resolution**: Identifies the calling shell and terminal emulator via Toolhelp32 process snapshot inspection.
- **Package Manager Support**: Counts installed packages for Winget, Scoop, Cargo, Chocolatey, and npm via direct directory indexing.
- **ASCII Art & Themes**: Built-in ASCII logos for Windows 11, Windows 10, Windows 7, Windows XP, and compact variants with configurable ANSI color themes.
- **Formats**: Supports standard two-column terminal output, a 5-line `--minimal` view, and structured `--json` output.
- **Self-Updating**: In-app `winfetch update` command for checking and replacing the binary from official releases.

---

## Installation

### Cargo
```bash
cargo install winfetch
```

### Pre-Compiled Binaries
Pre-built Windows binaries (`x86_64`) and MSI installers are available on the [GitHub Releases](https://github.com/realalden27/winfetch/releases) page.

---

## Usage

### Basic Execution
```bash
winfetch
```

### Minimal Summary
Displays a concise 5-line summary (OS, Host, CPU, GPU, Memory):
```bash
winfetch --minimal
```

### Logo Selection
Choose a specific ASCII logo or disable art:
```bash
winfetch --logo win11
winfetch --logo win10
winfetch --logo win7
winfetch --logo winxp
winfetch --logo small
winfetch --logo none
```

### Filter Specific Fields
Display only select information fields:
```bash
winfetch --only os,cpu,gpu,ram,disk
```

### Custom Accent Color
```bash
winfetch --color cyan    # cyan, blue, green, yellow, magenta, red, white
```

### JSON Output
Export raw system data for scripts, status bars, or automation:
```bash
winfetch --json
```

### Execution Timing
Measure collector and rendering duration:
```bash
winfetch --time
```

### In-App Update
Check for updates and replace the executable in-place:
```bash
winfetch update
```

---

## Options

| Flag | Description | Default |
| :--- | :--- | :--- |
| `-l, --logo <STYLE>` | Logo style (`auto`, `win11`, `win10`, `win7`, `winxp`, `small`, `none`) | `auto` |
| `-m, --minimal` | Display concise 5-line summary | `false` |
| `--json` | Output machine-readable JSON | `false` |
| `-t, --time` | Display internal execution time | `false` |
| `--only <FIELDS>` | Comma-separated list of fields to display | All |
| `-c, --color <COLOR>` | Accent color (`cyan`, `blue`, `green`, `yellow`, `magenta`, `red`, `white`) | `cyan` |
| `-h, --help` | Print help information | |
| `-V, --version` | Print version information | |

---

## Technical Details

| Subsystem | Collection Method | Notes |
| :--- | :--- | :--- |
| **Operating System** | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion` | Detects edition, display version (e.g. 24H2), and build |
| **Kernel** | `ntdll::RtlGetVersion` / `GetVersionExW` | Reads true kernel version without application manifest masking |
| **CPU** | `HKLM\HARDWARE\DESCRIPTION\System\CentralProcessor\0` | Processor brand string, base clock, and logical core counts |
| **GPU** | `HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-...}` | Adapter description and dedicated VRAM size |
| **Display** | Win32 `EnumDisplaySettingsW` | Screen resolution and vertical refresh rate (Hz) |
| **Memory** | Win32 `GlobalMemoryStatusEx` | Total physical RAM and currently utilized capacity |
| **Storage** | Win32 `GetLogicalDriveStringsW` & `GetDiskFreeSpaceExW` | Fixed local drives (`DRIVE_FIXED`) |
| **Battery** | Win32 `GetSystemPowerStatus` | Charge percentage and AC power connection state |
| **Terminal / Shell** | `CreateToolhelp32Snapshot` | Traverses parent process tree to resolve active terminal and shell |

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
