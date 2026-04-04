//! `Cli::apply_to` for `--sort name` with explicit `-u human`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_name_units_human() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "--units", "human"]);
    let mut p = Prefs {
        unit_mode: UnitMode::GiB,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.unit_mode, UnitMode::Human);
}

#[test]
fn short_sort_name_u_human_overrides_mib() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "-u", "human"]);
    let mut p = Prefs {
        unit_mode: UnitMode::MiB,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::Human);
}
