//! `Cli::apply_to` for `-r` (refresh) and `--no-compact`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_no_compact() {
    let cli = Cli::parse_from(["storageshower", "-r", "15", "--no-compact"]);
    let mut p = Prefs {
        compact: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 15);
    assert!(!p.compact);
}

#[test]
fn no_compact_only_preserves_refresh() {
    let cli = Cli::parse_from(["storageshower", "--no-compact"]);
    let mut p = Prefs {
        refresh_rate: 6,
        compact: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.compact);
    assert_eq!(p.refresh_rate, 6);
}
