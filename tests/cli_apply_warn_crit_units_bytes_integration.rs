//! `Cli::apply_to` for `-w`, `-C`, and `--units bytes`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::UnitMode;

#[test]
fn warn_crit_units_bytes() {
    let cli = Cli::parse_from(["storageshower", "-w", "58", "-C", "93", "--units", "bytes"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 58);
    assert_eq!(p.thresh_crit, 93);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}

#[test]
fn warn_only_units_bytes_preserves_crit() {
    let cli = Cli::parse_from(["storageshower", "-w", "41", "-u", "bytes"]);
    let mut p = Prefs {
        thresh_crit: 89,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 41);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
    assert_eq!(p.thresh_crit, 89);
}
