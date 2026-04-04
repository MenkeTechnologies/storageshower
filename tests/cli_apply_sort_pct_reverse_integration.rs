//! `Cli::apply_to` for `--sort pct` with `--reverse` (no column flags).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_pct_reverse() {
    let cli = Cli::parse_from(["storageshower", "--sort", "pct", "--reverse"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert!(p.sort_rev);
}

#[test]
fn short_sort_pct_r_sets_rev() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "-R"]);
    let mut p = Prefs {
        sort_rev: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert!(p.sort_rev);
}
