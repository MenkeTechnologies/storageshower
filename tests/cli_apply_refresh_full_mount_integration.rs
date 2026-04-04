//! `Cli::apply_to` for `-r` (refresh) and `-f` (full-mount).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_full_mount() {
    let cli = Cli::parse_from(["storageshower", "-r", "12", "-f"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 12);
    assert!(p.full_mount);
}

#[test]
fn full_mount_only_preserves_refresh() {
    let cli = Cli::parse_from(["storageshower", "-f"]);
    let mut p = Prefs {
        refresh_rate: 4,
        full_mount: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.full_mount);
    assert_eq!(p.refresh_rate, 4);
}
