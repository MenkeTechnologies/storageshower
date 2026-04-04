//! `Cli::apply_to` for `-w`, `-C`, and `--units mib`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::UnitMode;

#[test]
fn warn_crit_units_mib() {
    let cli = Cli::parse_from(["storageshower", "-w", "59", "-C", "95", "--units", "mib"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 59);
    assert_eq!(p.thresh_crit, 95);
    assert_eq!(p.unit_mode, UnitMode::MiB);
}

#[test]
fn crit_only_units_mib_preserves_warn() {
    let cli = Cli::parse_from(["storageshower", "-C", "87", "-u", "mib"]);
    let mut p = Prefs {
        thresh_warn: 52,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_crit, 87);
    assert_eq!(p.unit_mode, UnitMode::MiB);
    assert_eq!(p.thresh_warn, 52);
}
