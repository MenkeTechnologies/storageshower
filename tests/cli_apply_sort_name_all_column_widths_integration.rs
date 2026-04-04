//! `Cli::apply_to` for `--sort name` with all three column width flags.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name_with_mount_bar_end_pct_widths() {
    let cli = Cli::parse_from([
        "storageshower",
        "--sort",
        "name",
        "--col-mount",
        "17",
        "--col-bar-end",
        "29",
        "--col-pct",
        "12",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.col_mount_w, 17);
    assert_eq!(p.col_bar_end_w, 29);
    assert_eq!(p.col_pct_w, 12);
}

#[test]
fn short_sort_name_col_bar_end_only() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "--col-bar-end", "40"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.col_bar_end_w, 40);
}
