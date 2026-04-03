//! Deep `Cli::apply_to` stacks (many flags in one argv).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{BarStyle, ColorMode, SortMode, UnitMode};

fn parse(args: &[&str]) -> Cli {
    let mut v = vec!["storageshower"];
    v.extend_from_slice(args);
    Cli::try_parse_from(v).unwrap_or_else(|e| panic!("parse {:?}: {e}", args))
}

#[test]
fn stack_display_thresholds_columns_refresh() {
    let c = parse(&[
        "--sort",
        "size",
        "--reverse",
        "--warn",
        "55",
        "--crit",
        "88",
        "--refresh",
        "12",
        "--col-mount",
        "24",
        "--col-bar-end",
        "40",
        "--col-pct",
        "8",
    ]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.sort_rev);
    assert_eq!(p.thresh_warn, 55);
    assert_eq!(p.thresh_crit, 88);
    assert_eq!(p.refresh_rate, 12);
    assert_eq!(p.col_mount_w, 24);
    assert_eq!(p.col_bar_end_w, 40);
    assert_eq!(p.col_pct_w, 8);
}

#[test]
fn stack_units_bar_color_local_virtual() {
    let c = parse(&[
        "--units",
        "gib",
        "--bar-style",
        "thin",
        "--color",
        "purple",
        "--local-only",
        "--virtual",
    ]);
    let mut p = Prefs {
        show_all: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::GiB);
    assert_eq!(p.bar_style, BarStyle::Thin);
    assert_eq!(p.color_mode, ColorMode::Purple);
    assert!(p.show_local);
    assert!(p.show_all);
}

#[test]
fn stack_compact_full_mount_no_used_no_tooltips() {
    let c = parse(&["--compact", "--full-mount", "--no-used", "--no-tooltips"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.compact);
    assert!(p.full_mount);
    assert!(!p.show_used);
    assert!(!p.show_tooltips);
}

#[test]
fn stack_border_header_bars_used_positive() {
    let c = parse(&["--border", "--header", "--bars", "--used"]);
    let mut p = Prefs {
        show_border: false,
        show_header: false,
        show_bars: false,
        show_used: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_border);
    assert!(p.show_header);
    assert!(p.show_bars);
    assert!(p.show_used);
}

#[test]
fn stack_config_theme_sort() {
    let c = parse(&[
        "--config",
        "/tmp/ss.toml",
        "--theme",
        "palette",
        "--sort",
        "pct",
    ]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(c.config.as_deref(), Some("/tmp/ss.toml"));
    assert_eq!(p.active_theme.as_deref(), Some("palette"));
    assert_eq!(p.sort_mode, SortMode::Pct);
}

#[test]
fn stack_no_virtual_no_local_reverse() {
    let c = parse(&["--no-virtual", "--no-local", "--reverse"]);
    let mut p = Prefs {
        show_all: true,
        show_local: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.show_all);
    assert!(!p.show_local);
    assert!(p.sort_rev);
}

#[test]
fn stack_mib_bytes_mode_and_crit() {
    let c = parse(&["--units", "mib", "--crit", "95"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::MiB);
    assert_eq!(p.thresh_crit, 95);
}

#[test]
fn stack_ascii_bars_zaibatsu_color() {
    let c = parse(&["--bar-style", "ascii", "--color", "zaibatsu"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Ascii);
    assert_eq!(p.color_mode, ColorMode::Zaibatsu);
}

#[test]
fn stack_glitch_pop_and_sort_name() {
    let c = parse(&["--color", "glitch-pop", "--sort", "name"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::GlitchPop);
    assert_eq!(p.sort_mode, SortMode::Name);
}

#[test]
fn stack_deep_net_color_bytes_units() {
    let c = parse(&["--color", "deep-net", "--units", "bytes"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::DeepNet);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}
