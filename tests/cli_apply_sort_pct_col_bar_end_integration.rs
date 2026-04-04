//! `Cli::apply_to` for `--sort pct` with `--col-bar-end`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_pct_col_bar_end() {
    let cli = Cli::parse_from(["storageshower", "--sort", "pct", "--col-bar-end", "19"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.col_bar_end_w, 19);
}

#[test]
fn short_sort_pct_col_bar_end_preserves_mount_width() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "--col-bar-end", "7"]);
    let mut p = Prefs {
        col_mount_w: 31,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_bar_end_w, 7);
    assert_eq!(p.col_mount_w, 31);
}
