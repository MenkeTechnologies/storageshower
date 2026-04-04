//! `Cli::apply_to` for `--sort pct` with `-u human`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_pct_units_human() {
    let cli = Cli::parse_from(["storageshower", "--sort", "pct", "--units", "human"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.unit_mode, UnitMode::Human);
}

#[test]
fn short_sort_pct_u_human() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "-u", "human"]);
    let mut p = Prefs {
        unit_mode: UnitMode::GiB,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::Human);
}
