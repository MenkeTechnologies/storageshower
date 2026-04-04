//! `Cli::apply_to` for `-k` / `--compact` and `--refresh`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn compact_and_refresh() {
    let cli = Cli::parse_from(["storageshower", "-k", "--refresh", "12"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(p.compact);
    assert_eq!(p.refresh_rate, 12);
}

#[test]
fn compact_short_only() {
    let cli = Cli::parse_from(["storageshower", "--compact"]);
    let mut p = Prefs {
        compact: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.compact);
}
