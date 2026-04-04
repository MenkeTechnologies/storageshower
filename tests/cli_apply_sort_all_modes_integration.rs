//! `Cli::apply_to` for each `SortMode`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
}

#[test]
fn sort_pct() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
}

#[test]
fn sort_size() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
}
