//! `Cli::apply_to` for `--sort name` with thresholds and column widths.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name_warn_crit_col_mount_bar_end() {
    let cli = Cli::parse_from([
        "storageshower",
        "--sort",
        "name",
        "-w",
        "48",
        "-C",
        "96",
        "--col-mount",
        "19",
        "--col-bar-end",
        "31",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.thresh_warn, 48);
    assert_eq!(p.thresh_crit, 96);
    assert_eq!(p.col_mount_w, 19);
    assert_eq!(p.col_bar_end_w, 31);
}

#[test]
fn short_sort_name_col_pct() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "--col-pct", "14"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.col_pct_w, 14);
}
