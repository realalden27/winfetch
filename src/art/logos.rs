#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub enum LogoStyle {
    #[default]
    Auto,
    Win11,
    Win10,
    Win7,
    Winxp,
    Small,
    None,
}

pub struct Logo {
    pub lines: Vec<String>,
    pub width: usize,
}

const RESET: &str = "\x1b[0m";
const WIN_BLUE: &str = "\x1b[38;2;0;164;239m";
const WIN_RED: &str = "\x1b[38;2;242;80;34m";
const WIN_GREEN: &str = "\x1b[38;2;127;186;0m";
const WIN_YELLOW: &str = "\x1b[38;2;255;185;0m";

#[must_use]
pub fn get_logo(style: LogoStyle, is_win11: bool) -> Logo {
    let chosen = match style {
        LogoStyle::Auto => {
            if is_win11 {
                LogoStyle::Win11
            } else {
                LogoStyle::Win10
            }
        },
        other => other,
    };

    match chosen {
        LogoStyle::Win10 => logo_win10(),
        LogoStyle::Win7 => logo_win7(),
        LogoStyle::Winxp => logo_winxp(),
        LogoStyle::Small => logo_small(),
        LogoStyle::None => Logo {
            lines: Vec::new(),
            width: 0,
        },
        LogoStyle::Win11 | LogoStyle::Auto => logo_win11(),
    }
}

fn logo_win11() -> Logo {
    let red = WIN_RED;
    let grn = WIN_GREEN;
    let blu = WIN_BLUE;
    let yel = WIN_YELLOW;
    let r = RESET;
    let raw = vec![
        format!("{red}██████████{r}   {grn}██████████{r}"),
        format!("{red}██████████{r}   {grn}██████████{r}"),
        format!("{red}██████████{r}   {grn}██████████{r}"),
        format!("{red}██████████{r}   {grn}██████████{r}"),
        format!("{red}██████████{r}   {grn}██████████{r}"),
        String::new(),
        format!("{blu}██████████{r}   {yel}██████████{r}"),
        format!("{blu}██████████{r}   {yel}██████████{r}"),
        format!("{blu}██████████{r}   {yel}██████████{r}"),
        format!("{blu}██████████{r}   {yel}██████████{r}"),
        format!("{blu}██████████{r}   {yel}██████████{r}"),
    ];
    Logo { lines: raw, width: 23 }
}

fn logo_win10() -> Logo {
    let red = WIN_RED;
    let grn = WIN_GREEN;
    let blu = WIN_BLUE;
    let yel = WIN_YELLOW;
    let r = RESET;
    let raw = vec![
        format!("                                  {grn}.., {r}"),
        format!("                          {grn}....,,:;+ccllll{r}"),
        format!("            {red}...,,,+:;cllll{r}{grn}llllllllllllll{r}"),
        format!("    {red},ccllllllllllllll{r}    {grn}llllllllllllllll{r}"),
        format!("    {red}lllllllllllllllll{r}    {grn}llllllllllllllll{r}"),
        format!("    {red}lllllllllllllllll{r}    {grn}llllllllllllllll{r}"),
        format!("    {red}lllllllllllllllll{r}    {grn}llllllllllllllll{r}"),
        format!("    {red}lllllllllllllllll{r}    {grn}llllllllllllllll{r}"),
        String::new(),
        format!("    {blu}lllllllllllllllll{r}    {yel}llllllllllllllll{r}"),
        format!("    {blu}lllllllllllllllll{r}    {yel}llllllllllllllll{r}"),
        format!("    {blu}lllllllllllllllll{r}    {yel}llllllllllllllll{r}"),
        format!("    {blu}lllllllllllllllll{r}    {yel}llllllllllllllll{r}"),
        format!("    {blu}`'cclllllllllllll{r}    {yel}llllllllllllllll{r}"),
        format!("           {blu}`'\"\"*::clll{r} {yel}llllllllllllllll{r}"),
        format!("                      {yel}````''*::cllllllll{r}"),
    ];
    Logo { lines: raw, width: 40 }
}

fn logo_win7() -> Logo {
    let red = WIN_RED;
    let grn = WIN_GREEN;
    let blu = WIN_BLUE;
    let yel = WIN_YELLOW;
    let r = RESET;

    let raw = vec![
        format!("       {red}..,,;;;cccll{r} {grn}llccc;;;,,..{r}"),
        format!("   {red}.:clllllllllllll{r} {grn}lllllllllllll:{r}"),
        format!("  {red}:clllllllllllllll{r} {grn}lllllllllllllll:{r}"),
        format!(" {red}:cllllllllllllllll{r} {grn}llllllllllllllll:{r}"),
        format!(" {red}llllllllllllllllll{r} {grn}llllllllllllllllll{r}"),
        format!(" {red}llllllllllllllllll{r} {grn}llllllllllllllllll{r}"),
        String::new(),
        format!(" {blu}llllllllllllllllll{r} {yel}llllllllllllllllll{r}"),
        format!(" {blu}llllllllllllllllll{r} {yel}llllllllllllllllll{r}"),
        format!(" {blu}:cllllllllllllllll{r} {yel}llllllllllllllll:{r}"),
        format!("  {blu}:clllllllllllllll{r} {yel}lllllllllllllll:{r}"),
        format!("   {blu}.:clllllllllllll{r} {yel}lllllllllllll:{r}"),
        format!("       {blu}..,,;;;cccll{r} {yel}llccc;;;,,..{r}"),
    ];
    Logo { lines: raw, width: 37 }
}

fn logo_winxp() -> Logo {
    let red = WIN_RED;
    let grn = WIN_GREEN;
    let blu = WIN_BLUE;
    let yel = WIN_YELLOW;
    let r = RESET;

    let raw = vec![
        format!("                {red}..{r}"),
        format!("               {red}.o00o.{r}"),
        format!("             {red}.o0000000o.{r}"),
        format!("       {red}.o000000000000000o.{r}   {grn}.o0000000o.{r}"),
        format!("     {red}.o0000000000000000000o.{r} {grn}.o00000000000o.{r}"),
        format!("   {red}.o00000000000000000000000{r} {grn}.o0000000000000o.{r}"),
        format!("  {red}o0000000000000000000000000{r} {grn}o000000000000000o{r}"),
        format!("  {red}o0000000000000000000000000{r} {grn}o000000000000000o{r}"),
        String::new(),
        format!("  {blu}o0000000000000000000000000{r} {yel}o000000000000000o{r}"),
        format!("  {blu}o0000000000000000000000000{r} {yel}o000000000000000o{r}"),
        format!("   {blu}`o00000000000000000000000{r} {yel}`o0000000000000o'{r}"),
        format!("     {blu}`o0000000000000000000o'{r} {yel}`o00000000000o'{r}"),
        format!("       {blu}`o000000000000000o'{r}   {yel}`o0000000o'{r}"),
    ];
    Logo { lines: raw, width: 43 }
}

fn logo_small() -> Logo {
    let red = WIN_RED;
    let grn = WIN_GREEN;
    let blu = WIN_BLUE;
    let yel = WIN_YELLOW;
    let r = RESET;
    let raw = vec![
        format!("{red}████{r} {grn}████{r}"),
        format!("{red}████{r} {grn}████{r}"),
        format!("{blu}████{r} {yel}████{r}"),
        format!("{blu}████{r} {yel}████{r}"),
    ];
    Logo { lines: raw, width: 9 }
}
