//! `Cli::apply_to` for `-r` (refresh) and `-k` (compact).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_compact() {
    let cli = Cli::parse_from(["storageshower", "-r", "14", "-k"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 14);
    assert!(p.compact);
}

#[test]
fn compact_only_preserves_refresh() {
    let cli = Cli::parse_from(["storageshower", "-k"]);
    let mut p = Prefs {
        refresh_rate: 3,
        compact: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.compact);
    assert_eq!(p.refresh_rate, 3);
}
