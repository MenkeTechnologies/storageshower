//! `Cli::apply_to` for `--sort name` and `--refresh`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name_refresh_sixty() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "--refresh", "60"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.refresh_rate, 60);
}

#[test]
fn short_sort_and_refresh() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "-r", "2"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 2);
}
