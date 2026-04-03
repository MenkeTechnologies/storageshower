//! Miscellaneous `Cli::parse` + `apply_to` combinations.

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
fn apply_used_flag_enables_show_used() {
    let c = parse(&["--used"]);
    let mut p = Prefs {
        show_used: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_used);
}

#[test]
fn apply_tooltips_flag() {
    let c = parse(&["--tooltips"]);
    let mut p = Prefs {
        show_tooltips: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_tooltips);
}

#[test]
fn apply_header_flag() {
    let c = parse(&["--header"]);
    let mut p = Prefs {
        show_header: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_header);
}

#[test]
fn apply_border_flag() {
    let c = parse(&["--border"]);
    let mut p = Prefs {
        show_border: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_border);
}

#[test]
fn apply_bars_flag() {
    let c = parse(&["--bars"]);
    let mut p = Prefs {
        show_bars: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_bars);
}

#[test]
fn apply_virtual_flag() {
    let c = parse(&["--virtual"]);
    let mut p = Prefs {
        show_all: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_all);
}

#[test]
fn apply_no_used_after_used() {
    let c = parse(&["--used", "--no-used"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(!p.show_used);
}

#[test]
fn apply_no_tooltips_after_tooltips() {
    let c = parse(&["--tooltips", "--no-tooltips"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(!p.show_tooltips);
}

#[test]
fn apply_sort_pct_and_units_human() {
    let c = parse(&["--sort", "pct", "--units", "human"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.unit_mode, UnitMode::Human);
}

#[test]
fn apply_bar_style_gradient_explicit() {
    let c = parse(&["--bar-style", "gradient"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Gradient);
}

#[test]
fn apply_color_sakura() {
    let c = parse(&["--color", "sakura"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Sakura);
}

#[test]
fn apply_color_sunset() {
    let c = parse(&["--color", "sunset"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Sunset);
}

#[test]
fn apply_warn_only() {
    let c = parse(&["--warn", "42"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 42);
    assert_eq!(p.thresh_crit, 90);
}

#[test]
fn apply_crit_only() {
    let c = parse(&["--crit", "80"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.thresh_crit, 80);
    assert_eq!(p.thresh_warn, 70);
}
