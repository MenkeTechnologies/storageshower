//! `Cli::apply_to` for each `--units` value.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::UnitMode;

#[test]
fn units_human() {
    let cli = Cli::parse_from(["storageshower", "--units", "human"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::Human);
}

#[test]
fn units_gib() {
    let cli = Cli::parse_from(["storageshower", "-u", "gib"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::GiB);
}

#[test]
fn units_mib() {
    let cli = Cli::parse_from(["storageshower", "--units", "mib"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::MiB);
}

#[test]
fn units_bytes() {
    let cli = Cli::parse_from(["storageshower", "--units", "bytes"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}
