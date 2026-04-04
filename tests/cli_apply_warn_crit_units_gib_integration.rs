//! `Cli::apply_to` for `-w`, `-C`, and `--units gib`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::UnitMode;

#[test]
fn warn_crit_units_gib() {
    let cli = Cli::parse_from(["storageshower", "-w", "64", "-C", "98", "--units", "gib"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 64);
    assert_eq!(p.thresh_crit, 98);
    assert_eq!(p.unit_mode, UnitMode::GiB);
}

#[test]
fn warn_only_units_gib_preserves_crit() {
    let cli = Cli::parse_from(["storageshower", "-w", "37", "-u", "gib"]);
    let mut p = Prefs {
        thresh_crit: 91,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 37);
    assert_eq!(p.unit_mode, UnitMode::GiB);
    assert_eq!(p.thresh_crit, 91);
}
