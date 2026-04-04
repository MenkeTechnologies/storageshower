//! `Cli::apply_to` for `-u` (units) and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::UnitMode;

#[test]
fn unit_mode_gib_no_local() {
    let cli = Cli::parse_from(["storageshower", "-u", "gib", "--no-local"]);
    let mut p = Prefs {
        unit_mode: UnitMode::Human,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::GiB);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_unit_mode() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        unit_mode: UnitMode::MiB,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.unit_mode, UnitMode::MiB);
}
