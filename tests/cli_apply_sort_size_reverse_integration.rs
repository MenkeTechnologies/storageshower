//! `Cli::apply_to` for `--sort size` with `--reverse` (no compact).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_reverse() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size", "--reverse"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.sort_rev);
}

#[test]
fn short_sort_size_r_sets_rev() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "-R"]);
    let mut p = Prefs {
        sort_rev: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.sort_rev);
}
