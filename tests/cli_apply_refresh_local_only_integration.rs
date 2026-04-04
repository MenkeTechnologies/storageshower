//! `Cli::apply_to` for `-r` (refresh) and `-l` (local-only).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_local_only() {
    let cli = Cli::parse_from(["storageshower", "-r", "11", "-l"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 11);
    assert!(p.show_local);
}

#[test]
fn short_local_only_preserves_refresh() {
    let cli = Cli::parse_from(["storageshower", "-l"]);
    let mut p = Prefs {
        refresh_rate: 2,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_local);
    assert_eq!(p.refresh_rate, 2);
}
