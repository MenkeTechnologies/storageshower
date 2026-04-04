//! `Cli::apply_to` for `--sort size` with `-u mib`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_size_units_mib() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size", "-u", "mib"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.unit_mode, UnitMode::MiB);
}

#[test]
fn short_sort_size_mib_overrides_bytes() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--units", "mib"]);
    let mut p = Prefs {
        unit_mode: UnitMode::Bytes,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::MiB);
}
