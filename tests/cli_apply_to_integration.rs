//! `Cli::apply_to(&mut Prefs)` — integration coverage of CLI → persisted prefs mapping.

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
fn apply_sort_mode_size() {
    let c = parse(&["--sort", "size"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
}

#[test]
fn apply_refresh_rate() {
    let c = parse(&["--refresh", "7"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 7);
}

#[test]
fn apply_bar_style_thin() {
    let c = parse(&["--bar-style", "thin"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Thin);
}

#[test]
fn apply_color_plasma_core() {
    let c = parse(&["--color", "plasma-core"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::PlasmaCore);
}

#[test]
fn apply_warn_crit() {
    let c = parse(&["--warn", "61", "--crit", "92"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 61);
    assert_eq!(p.thresh_crit, 92);
}

#[test]
fn apply_units_bytes() {
    let c = parse(&["--units", "bytes"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}

#[test]
fn apply_column_widths() {
    let c = parse(&["--col-mount", "18", "--col-bar-end", "35", "--col-pct", "6"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 18);
    assert_eq!(p.col_bar_end_w, 35);
    assert_eq!(p.col_pct_w, 6);
}

#[test]
fn apply_sort_rev_true() {
    let c = parse(&["--reverse"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.sort_rev);
}

#[test]
fn apply_no_reverse_clears_sort_rev() {
    let c = parse(&["--reverse", "--no-reverse"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(!p.sort_rev);
}

#[test]
fn apply_local_only() {
    let c = parse(&["--local-only"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.show_local);
}

#[test]
fn apply_no_local() {
    let c = parse(&["--local-only", "--no-local"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(!p.show_local);
}

#[test]
fn apply_compact_and_no_compact() {
    let c = parse(&["--compact", "--no-compact"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(!p.compact);
}

#[test]
fn apply_full_mount_flags() {
    let c = parse(&["--full-mount", "--no-full-mount"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(!p.full_mount);
}

#[test]
fn apply_bars_flags() {
    let c = parse(&["--no-bars", "--bars"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.show_bars);
}

#[test]
fn apply_border_flags() {
    let c = parse(&["--no-border", "--border"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.show_border);
}

#[test]
fn apply_header_flags() {
    let c = parse(&["--no-header", "--header"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.show_header);
}

#[test]
fn apply_used_flags() {
    let c = parse(&["--no-used", "--used"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.show_used);
}

#[test]
fn apply_tooltips_flags() {
    let c = parse(&["--no-tooltips", "--tooltips"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.show_tooltips);
}

#[test]
fn apply_virtual_sets_show_all_true() {
    let c = parse(&["--virtual"]);
    let mut p = Prefs {
        show_all: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_all);
}

#[test]
fn apply_no_virtual_sets_show_all_false() {
    let c = parse(&["--no-virtual"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(!p.show_all);
}

#[test]
fn apply_theme_name() {
    let c = parse(&["--theme", "saved-palette"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("saved-palette"));
}

#[test]
fn apply_stack_does_not_touch_unset_options() {
    let c = parse(&["--sort", "pct"]);
    let mut p = Prefs {
        refresh_rate: 99,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.refresh_rate, 99);
}

#[test]
fn apply_overwrites_refresh_when_provided() {
    let c = parse(&["--refresh", "3"]);
    let mut p = Prefs {
        refresh_rate: 99,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 3);
}
