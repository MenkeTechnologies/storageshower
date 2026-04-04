//! `Cli::apply_to` for `--sort name` with `--col-bar-end`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name_col_bar_end() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "--col-bar-end", "25"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.col_bar_end_w, 25);
}

#[test]
fn short_sort_name_col_bar_end_preserves_mount_width() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "--col-bar-end", "12"]);
    let mut p = Prefs {
        col_mount_w: 17,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_bar_end_w, 12);
    assert_eq!(p.col_mount_w, 17);
}
