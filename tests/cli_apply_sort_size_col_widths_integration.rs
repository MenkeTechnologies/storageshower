//! `Cli::apply_to` for `--sort size` with column width flags.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_with_three_column_widths() {
    let cli = Cli::parse_from([
        "storageshower",
        "--sort",
        "size",
        "--col-mount",
        "18",
        "--col-bar-end",
        "22",
        "--col-pct",
        "7",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.col_mount_w, 18);
    assert_eq!(p.col_bar_end_w, 22);
    assert_eq!(p.col_pct_w, 7);
}

#[test]
fn short_sort_size_col_pct_only() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--col-pct", "11"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.col_pct_w, 11);
}
