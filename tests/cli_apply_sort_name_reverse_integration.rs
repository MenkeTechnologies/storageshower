//! `Cli::apply_to` for `--sort name` with `--reverse`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name_reverse() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "--reverse"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert!(p.sort_rev);
}

#[test]
fn short_sort_name_r_sets_rev() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "-R"]);
    let mut p = Prefs {
        sort_rev: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert!(p.sort_rev);
}
