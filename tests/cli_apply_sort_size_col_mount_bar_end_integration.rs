//! `Cli::apply_to` for `--sort size` with `--col-mount` and `--col-bar-end`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_col_mount_bar_end() {
    let cli = Cli::parse_from([
        "storageshower",
        "--sort",
        "size",
        "--col-mount",
        "23",
        "--col-bar-end",
        "41",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.col_mount_w, 23);
    assert_eq!(p.col_bar_end_w, 41);
}

#[test]
fn short_sort_size_col_mount_only() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--col-mount", "8"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.col_mount_w, 8);
}
