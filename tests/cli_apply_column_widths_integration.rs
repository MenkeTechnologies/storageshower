//! `Cli::apply_to` for `--col-mount`, `--col-bar-end`, `--col-pct`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn apply_all_three_column_flags() {
    let cli = Cli::parse_from([
        "storageshower",
        "--col-mount",
        "21",
        "--col-bar-end",
        "35",
        "--col-pct",
        "9",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 21);
    assert_eq!(p.col_bar_end_w, 35);
    assert_eq!(p.col_pct_w, 9);
}

#[test]
fn apply_col_mount_only_leaves_others_default() {
    let cli = Cli::parse_from(["storageshower", "--col-mount", "14"]);
    let mut p = Prefs {
        col_bar_end_w: 99,
        col_pct_w: 88,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 14);
    assert_eq!(p.col_bar_end_w, 99);
    assert_eq!(p.col_pct_w, 88);
}

#[test]
fn apply_col_pct_with_other_prefs_intact() {
    let cli = Cli::parse_from(["storageshower", "--col-pct", "6"]);
    let mut p = Prefs {
        thresh_warn: 55,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_pct_w, 6);
    assert_eq!(p.thresh_warn, 55);
}
