//! `Cli::apply_to` on top of non-default `Prefs` (simulates config + CLI overrides).

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
fn cli_overrides_existing_sort_mode() {
    let c = parse(&["--sort", "size"]);
    let mut p = Prefs {
        sort_mode: SortMode::Pct,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
}

#[test]
fn cli_overrides_existing_color_mode() {
    let c = parse(&["--color", "amber"]);
    let mut p = Prefs {
        color_mode: ColorMode::Matrix,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Amber);
}

#[test]
fn cli_overrides_unit_mode_from_gib_to_mib() {
    let c = parse(&["--units", "mib"]);
    let mut p = Prefs {
        unit_mode: UnitMode::GiB,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::MiB);
}

#[test]
fn cli_refresh_overrides_high_existing() {
    let c = parse(&["--refresh", "2"]);
    let mut p = Prefs {
        refresh_rate: 100,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 2);
}

#[test]
fn cli_bar_style_overrides_existing() {
    let c = parse(&["--bar-style", "thin"]);
    let mut p = Prefs {
        bar_style: BarStyle::Ascii,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Thin);
}

#[test]
fn empty_parse_leaves_prefs_unchanged() {
    let c = parse(&[]);
    let mut p = Prefs {
        sort_mode: SortMode::Size,
        sort_rev: true,
        thresh_warn: 55,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.sort_rev);
    assert_eq!(p.thresh_warn, 55);
}

#[test]
fn cli_warn_crit_override_previous_thresholds() {
    let c = parse(&["--warn", "60", "--crit", "95"]);
    let mut p = Prefs {
        thresh_warn: 10,
        thresh_crit: 20,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 60);
    assert_eq!(p.thresh_crit, 95);
}

#[test]
fn cli_reverse_toggles_off_existing_true() {
    let c = parse(&["--no-reverse"]);
    let mut p = Prefs {
        sort_rev: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.sort_rev);
}
