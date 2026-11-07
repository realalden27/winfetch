# Changelog

All notable changes to `winfetch` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-11-07

### Changed
- Refined documentation and CLI metadata with concise, professional technical descriptions.
- Added comprehensive technical subsystem table in README detailing Win32 APIs and collection methods.

## [0.1.0] - 2026-11-07

### Added
- **Native Windows System Fetch**: Pure Win32 registry reads and kernel APIs without WMI/COM subsystems.
- **Non-Blocking Multi-Threaded Architecture**: All hardware, OS, disk, and package collectors execute concurrently via Rayon.
- **Accurate Windows 11 & 24H2 Detection**: Resolves registry backwards-compatibility quirks to accurately identify Windows 11, edition, build number, and UBR.
- **Hardware Metrics**: Motherboard model, CPU clock and core counts, display adapter enumeration with dedicated VRAM, and RAM usage.
- **Display Details**: Primary screen resolution and refresh rate (e.g. 1920x1080 @ 165Hz) via `EnumDisplaySettingsW`.
- **Environment & Storage**: Toolhelp32 process snapshot shell/terminal resolution, battery percentage and charging status, fixed disk space bars.
- **Visual ASCII Art**: Clean logos for Windows 11, Windows 10, Windows 7, Windows XP, and compact mini-logos with ANSI palettes.
- **Automatic First-Run PATH Registration**: Copies executable to `%LOCALAPPDATA%\Programs\winfetch` and registers permanent User PATH.
- **In-App Self-Updater (`winfetch update`)**: Dual crates.io and GitHub Releases inspection with atomic in-place binary swapping.
- **CLI Options**: `--minimal`, `--json`, `--time`, `--only`, `--color`, and `--logo`.
