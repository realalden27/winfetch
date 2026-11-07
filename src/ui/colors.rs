pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[90m";

pub const C_RED: &str = "\x1b[1;91m";
pub const C_GREEN: &str = "\x1b[1;92m";
pub const C_YELLOW: &str = "\x1b[1;93m";
pub const C_BLUE: &str = "\x1b[1;94m";
pub const C_MAGENTA: &str = "\x1b[1;95m";
pub const C_CYAN: &str = "\x1b[1;96m";
pub const C_WHITE: &str = "\x1b[1;97m";

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub enum AccentColor {
    #[default]
    Multi,
    Cyan,
    Blue,
    Green,
    Yellow,
    Magenta,
    Red,
    White,
}

impl AccentColor {
    #[must_use]
    pub fn ansi_code(&self) -> &'static str {
        match self {
            AccentColor::Multi | AccentColor::Cyan => "\x1b[36m",
            AccentColor::Blue => "\x1b[34m",
            AccentColor::Green => "\x1b[32m",
            AccentColor::Yellow => "\x1b[33m",
            AccentColor::Magenta => "\x1b[35m",
            AccentColor::Red => "\x1b[31m",
            AccentColor::White => "\x1b[37m",
        }
    }

    #[must_use]
    pub fn bold_code(&self) -> String {
        match self {
            AccentColor::Multi | AccentColor::Cyan => C_CYAN.to_string(),
            AccentColor::Blue => C_BLUE.to_string(),
            AccentColor::Green => C_GREEN.to_string(),
            AccentColor::Yellow => C_YELLOW.to_string(),
            AccentColor::Magenta => C_MAGENTA.to_string(),
            AccentColor::Red => C_RED.to_string(),
            AccentColor::White => C_WHITE.to_string(),
        }
    }
}
