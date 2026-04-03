//! Edge values for column widths and thresholds (`Cli::apply_to`).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

fn parse(args: &[&str]) -> Cli {
    let mut v = vec!["storageshower"];
    v.extend_from_slice(args);
    Cli::try_parse_from(v).unwrap_or_else(|e| panic!("parse {:?}: {e}", args))
}

#[test]
fn col_mount_zero_means_auto() {
    let c = parse(&["--col-mount", "0"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 0);
}

#[test]
fn col_bar_end_zero_means_auto() {
    let c = parse(&["--col-bar-end", "0"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.col_bar_end_w, 0);
}

#[test]
fn col_pct_zero_means_auto() {
    let c = parse(&["--col-pct", "0"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.col_pct_w, 0);
}

#[test]
fn all_three_column_widths_max_u16() {
    let c = parse(&[
        "--col-mount",
        "65535",
        "--col-bar-end",
        "60000",
        "--col-pct",
        "500",
    ]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 65535);
    assert_eq!(p.col_bar_end_w, 60000);
    assert_eq!(p.col_pct_w, 500);
}

#[test]
fn warn_zero_crit_100() {
    let c = parse(&["--warn", "0", "--crit", "100"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 0);
    assert_eq!(p.thresh_crit, 100);
}

#[test]
fn warn_crit_equal_allowed() {
    let c = parse(&["--warn", "75", "--crit", "75"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 75);
    assert_eq!(p.thresh_crit, 75);
}

#[test]
fn refresh_very_large() {
    let c = parse(&["--refresh", "86400"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 86400);
}

#[test]
fn stacked_widths_with_sort_and_units() {
    let c = parse(&[
        "--col-mount",
        "30",
        "--col-bar-end",
        "28",
        "--col-pct",
        "7",
        "--sort",
        "pct",
        "--units",
        "human",
    ]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 30);
    assert_eq!(p.col_bar_end_w, 28);
    assert_eq!(p.col_pct_w, 7);
    assert_eq!(p.sort_mode, storageshower::types::SortMode::Pct);
}
