//! `Cli::apply_to` for `--sort pct` with `-u mib`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_pct_units_mib() {
    let cli = Cli::parse_from(["storageshower", "--sort", "pct", "-u", "mib"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.unit_mode, UnitMode::MiB);
}

#[test]
fn short_sort_pct_mib_overrides_gib() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "--units", "mib"]);
    let mut p = Prefs {
        unit_mode: UnitMode::GiB,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::MiB);
}
