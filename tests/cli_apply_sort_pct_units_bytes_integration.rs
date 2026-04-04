//! `Cli::apply_to` for `--sort pct` with `-u bytes`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_pct_units_bytes() {
    let cli = Cli::parse_from(["storageshower", "--sort", "pct", "-u", "bytes"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}

#[test]
fn short_sort_pct_bytes_overrides_mib() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "--units", "bytes"]);
    let mut p = Prefs {
        unit_mode: UnitMode::MiB,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}
