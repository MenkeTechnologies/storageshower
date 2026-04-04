//! `Cli::apply_to` for `--sort name` with `-u bytes`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_name_units_bytes() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "--units", "bytes"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}

#[test]
fn short_sort_name_u_bytes() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "-u", "bytes"]);
    let mut p = Prefs {
        unit_mode: UnitMode::Human,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}
