//! `Cli::apply_to` for `--sort size` with `--col-bar-end`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_col_bar_end() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size", "--col-bar-end", "23"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.col_bar_end_w, 23);
}

#[test]
fn short_sort_size_col_bar_end_preserves_pct_width() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--col-bar-end", "6"]);
    let mut p = Prefs {
        col_pct_w: 14,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_bar_end_w, 6);
    assert_eq!(p.col_pct_w, 14);
}
