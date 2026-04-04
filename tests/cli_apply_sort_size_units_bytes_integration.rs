//! `Cli::apply_to` for `--sort size` with `-u bytes`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_size_units_bytes() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size", "-u", "bytes"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}

#[test]
fn short_sort_size_bytes_overrides_gib() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--units", "bytes"]);
    let mut p = Prefs {
        unit_mode: UnitMode::GiB,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}
