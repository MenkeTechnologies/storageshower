//! `Cli::apply_to` for `--sort pct` with `--col-mount`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_pct_col_mount() {
    let cli = Cli::parse_from(["storageshower", "--sort", "pct", "--col-mount", "21"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.col_mount_w, 21);
}

#[test]
fn short_sort_pct_col_mount_preserves_pct_width() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "--col-mount", "4"]);
    let mut p = Prefs {
        col_pct_w: 11,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 4);
    assert_eq!(p.col_pct_w, 11);
}
