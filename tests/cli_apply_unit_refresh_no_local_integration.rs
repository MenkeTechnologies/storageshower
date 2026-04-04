//! `Cli::apply_to` for `-u` (units), `-r` (refresh), and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::UnitMode;

#[test]
fn unit_mib_refresh_no_local() {
    let cli = Cli::parse_from(["storageshower", "-u", "mib", "-r", "4", "--no-local"]);
    let mut p = Prefs {
        unit_mode: UnitMode::Human,
        refresh_rate: 1,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::MiB);
    assert_eq!(p.refresh_rate, 4);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_unit_and_refresh() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        unit_mode: UnitMode::GiB,
        refresh_rate: 12,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.unit_mode, UnitMode::GiB);
    assert_eq!(p.refresh_rate, 12);
}
