//! `Cli::apply_to` for `--sort name` with `--col-mount`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name_col_mount() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "--col-mount", "27"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.col_mount_w, 27);
}

#[test]
fn short_sort_name_col_mount_preserves_bar_end_width() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "--col-mount", "9"]);
    let mut p = Prefs {
        col_bar_end_w: 18,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 9);
    assert_eq!(p.col_bar_end_w, 18);
}
