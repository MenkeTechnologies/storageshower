//! Additional `Cli::apply_to` bundles (flags that interact or stack).

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
fn apply_sort_pct_and_reverse_together() {
    let c = parse(&["--sort", "pct", "--reverse"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert!(p.sort_rev);
}

#[test]
fn apply_sort_size_and_reverse() {
    let c = parse(&["--sort", "size", "-R"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.sort_rev);
}

#[test]
fn apply_refresh_zero_warn_crit_extremes() {
    let c = parse(&["--refresh", "0", "--warn", "1", "--crit", "99"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 0);
    assert_eq!(p.thresh_warn, 1);
    assert_eq!(p.thresh_crit, 99);
}

#[test]
fn apply_color_and_bar_style_together() {
    let c = parse(&["--color", "matrix", "--bar-style", "ascii"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Matrix);
    assert_eq!(p.bar_style, BarStyle::Ascii);
}

#[test]
fn apply_units_gib_and_column_widths() {
    let c = parse(&["--units", "gib", "--col-mount", "20", "--col-pct", "8"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::GiB);
    assert_eq!(p.col_mount_w, 20);
    assert_eq!(p.col_pct_w, 8);
}

#[test]
fn apply_theme_name_with_color_does_not_panic() {
    let c = parse(&["--theme", "slot-a", "--color", "cyan"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("slot-a"));
    assert_eq!(p.color_mode, ColorMode::Cyan);
}

#[test]
fn apply_local_only_and_compact() {
    let c = parse(&["--local-only", "--compact"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.show_local);
    assert!(p.compact);
}

#[test]
fn apply_full_mount_and_no_used() {
    let c = parse(&["--full-mount", "--no-used"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.full_mount);
    assert!(!p.show_used);
}

#[test]
fn apply_no_virtual_after_virtual() {
    let c = parse(&["--virtual", "--no-virtual"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(!p.show_all);
}

#[test]
fn apply_virtual_alone() {
    let c = parse(&["--virtual"]);
    let mut p = Prefs {
        show_all: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_all);
}

#[test]
fn apply_border_header_bars_triple() {
    let c = parse(&["--no-border", "--no-header", "--no-bars"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(!p.show_border);
    assert!(!p.show_header);
    assert!(!p.show_bars);
}

#[test]
fn apply_config_path_with_sort() {
    let c = parse(&["--config", "/tmp/x.conf", "--sort", "name"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
}

#[test]
fn apply_neon_noir_hyphenated_color() {
    let c = parse(&["--color", "neon-noir"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::NeonNoir);
}

#[test]
fn apply_blade_runner_hyphenated() {
    let c = parse(&["--color", "blade-runner"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::BladeRunner);
}
