//! `Cli::apply_to` for `-w`, `-C`, and `--units human`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::UnitMode;

#[test]
fn warn_crit_units_human() {
    let cli = Cli::parse_from(["storageshower", "-w", "66", "-C", "97", "--units", "human"]);
    let mut p = Prefs {
        unit_mode: UnitMode::Bytes,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 66);
    assert_eq!(p.thresh_crit, 97);
    assert_eq!(p.unit_mode, UnitMode::Human);
}

#[test]
fn crit_only_units_human_preserves_warn() {
    let cli = Cli::parse_from(["storageshower", "-C", "86", "-u", "human"]);
    let mut p = Prefs {
        thresh_warn: 49,
        unit_mode: UnitMode::GiB,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_crit, 86);
    assert_eq!(p.unit_mode, UnitMode::Human);
    assert_eq!(p.thresh_warn, 49);
}
