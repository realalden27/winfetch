use clap::{Parser, Subcommand};

use crate::art::LogoStyle;
use crate::ui::AccentColor;

#[derive(Parser, Debug)]
#[command(
    name = "winfetch",
    author = "RealAlden27",
    version,
    about = "A lightweight, zero-WMI system information tool for Windows written in Rust.",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Select visual ASCII logo style
    #[arg(short, long, value_enum, default_value_t = LogoStyle::Auto)]
    pub logo: LogoStyle,

    /// Display concise minimal 5-line summary
    #[arg(short, long)]
    pub minimal: bool,

    /// Output machine-readable JSON for scripts, status bars, and terminal multiplexers
    #[arg(long)]
    pub json: bool,

    /// Measure and display internal collector and rendering time in milliseconds
    #[arg(short, long)]
    pub time: bool,

    /// Filter to display only specified fields (e.g. os,cpu,gpu,ram,disk,packages)
    #[arg(long)]
    pub only: Option<String>,

    /// Accent color theme for headers and labels
    #[arg(short, long, value_enum, default_value_t = AccentColor::Cyan)]
    pub color: AccentColor,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check crates.io & GitHub Releases and self-replace binary to the latest version
    Update,
}
