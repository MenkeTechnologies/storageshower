//! `Cli::apply_to` for `--sort pct` with `--col-pct` (no reverse).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_pct_col_pct() {
    let cli = Cli::parse_from(["storageshower", "--sort", "pct", "--col-pct", "17"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.col_pct_w, 17);
    assert!(!p.sort_rev);
}

#[test]
fn short_sort_pct_col_pct_preserves_bar_end_width() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "--col-pct", "8"]);
    let mut p = Prefs {
        col_bar_end_w: 20,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_pct_w, 8);
    assert_eq!(p.col_bar_end_w, 20);
}
