//! `Cli::apply_to` for `--sort size` with `--col-mount` (no bar-end).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_col_mount() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size", "--col-mount", "26"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.col_mount_w, 26);
}

#[test]
fn short_sort_size_col_mount_preserves_bar_end_width() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--col-mount", "5"]);
    let mut p = Prefs {
        col_bar_end_w: 40,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 5);
    assert_eq!(p.col_bar_end_w, 40);
}
