//! `Cli::apply_to` for `--sort name` with `--col-pct`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name_col_pct() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "--col-pct", "13"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.col_pct_w, 13);
}

#[test]
fn short_sort_name_col_pct_preserves_mount_width() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "--col-pct", "3"]);
    let mut p = Prefs {
        col_mount_w: 29,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_pct_w, 3);
    assert_eq!(p.col_mount_w, 29);
}
