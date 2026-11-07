#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashSet;
use std::time::Instant;

use winfetch::art::{LogoStyle, get_logo};
use winfetch::collectors::environment::format_uptime;
use winfetch::collectors::storage::render_progress_bar;
use winfetch::collectors::{PackageCounts, collect_all_parallel};
use winfetch::path_install::parse_version;
use winfetch::ui::{AccentColor, FormatOptions, format_fetch};

#[test]
fn test_parse_version() {
    assert_eq!(parse_version("0.1.0"), Some((0, 1, 0)));
    assert_eq!(parse_version("v1.2.3"), Some((1, 2, 3)));
    assert_eq!(parse_version("winfetch 0.1.4"), Some((0, 1, 4)));
    assert_eq!(parse_version("winfetch v2.0.0-rc.1"), Some((2, 0, 0)));
    assert_eq!(parse_version("invalid"), None);
}

#[test]
fn test_format_uptime() {
    assert_eq!(format_uptime(45), "0 mins");
    assert_eq!(format_uptime(125), "2 mins");
    assert_eq!(format_uptime(3665), "1 hours, 1 mins");
    assert_eq!(format_uptime(90060), "1 days, 1 hours, 1 mins");
}

#[test]
fn test_render_progress_bar() {
    let p0 = render_progress_bar(0, 10);
    assert_eq!(p0, "[          ]");

    let p50 = render_progress_bar(50, 10);
    assert_eq!(p50, "[=====     ]");

    let p100 = render_progress_bar(100, 10);
    assert_eq!(p100, "[==========]");

    let p120 = render_progress_bar(120, 10);
    assert_eq!(p120, "[==========]");
}

#[test]
fn test_package_counts_display() {
    let mut pkgs = PackageCounts::default();
    assert_eq!(pkgs.to_display_string(), None);

    pkgs.cargo = Some(15);
    pkgs.winget = Some(40);
    assert_eq!(pkgs.to_display_string(), Some("40 (winget), 15 (cargo)".to_string()));
}

#[test]
fn test_logos_structure() {
    let logo11 = get_logo(LogoStyle::Win11, true);
    assert!(!logo11.lines.is_empty());
    assert_eq!(logo11.width, 23);

    let logo10 = get_logo(LogoStyle::Win10, false);
    assert!(!logo10.lines.is_empty());
    assert_eq!(logo10.width, 40);

    let logo_small = get_logo(LogoStyle::Small, true);
    assert_eq!(logo_small.width, 9);

    let logo_none = get_logo(LogoStyle::None, true);
    assert!(logo_none.lines.is_empty());
    assert_eq!(logo_none.width, 0);
}

#[test]
fn test_collect_all_parallel_benchmark() {
    let start = Instant::now();
    let info = collect_all_parallel();
    let elapsed = start.elapsed();

    // Verify critical data populated
    assert!(!info.os.display_name.is_empty());
    assert!(!info.cpu.is_empty());
    assert!(!info.environment.user_at_host.is_empty());

    // Strict non-blocking guarantee: must execute well under 100ms even on cold run
    assert!(
        elapsed.as_millis() < 100,
        "Collection took too long: {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_formatter_rendering() {
    let info = collect_all_parallel();
    let logo = get_logo(LogoStyle::Win11, true);

    let opts = FormatOptions {
        color: AccentColor::Cyan,
        minimal: false,
        fields_filter: None,
    };

    let rendered = format_fetch(&info, &logo, &opts);
    assert!(rendered.contains("OS:"));
    assert!(rendered.contains("CPU:"));
    assert!(rendered.contains("Memory:"));

    // Minimal mode test
    let min_opts = FormatOptions {
        color: AccentColor::Blue,
        minimal: true,
        fields_filter: None,
    };
    let min_rendered = format_fetch(&info, &logo, &min_opts);
    assert!(min_rendered.contains("OS:"));
    assert!(min_rendered.lines().count() >= 5);

    // Filter test
    let mut filter = HashSet::new();
    filter.insert("cpu".to_string());
    let filter_opts = FormatOptions {
        color: AccentColor::Green,
        minimal: false,
        fields_filter: Some(filter),
    };
    let filter_rendered = format_fetch(&info, &logo, &filter_opts);
    assert!(filter_rendered.contains("CPU:"));
    assert!(!filter_rendered.contains("Memory:"));
}

#[test]
fn test_json_serialization() {
    let info = collect_all_parallel();
    let json = serde_json::to_string(&info).expect("JSON serialization failed");
    assert!(json.contains("display_name"));
    assert!(json.contains("cpu"));
    assert!(json.contains("memory"));
}
