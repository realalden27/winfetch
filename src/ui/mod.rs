pub mod colors;
pub mod formatter;

pub use colors::{AccentColor, BOLD, RESET};
pub use formatter::{FormatOptions, format_fetch, render_ansi_palette};
