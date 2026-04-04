//! `Cli::apply_to` for `--sort pct` with `-u gib`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_pct_units_gib() {
    let cli = Cli::parse_from(["storageshower", "--sort", "pct", "-u", "gib"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.unit_mode, UnitMode::GiB);
}

#[test]
fn short_sort_pct_gib_overrides_mib() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "--units", "gib"]);
    let mut p = Prefs {
        unit_mode: UnitMode::MiB,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::GiB);
}
