//! `Cli::apply_to` for `--warn` / `--crit`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn apply_warn_and_crit() {
    let cli = Cli::parse_from(["storageshower", "--warn", "12", "--crit", "88"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 12);
    assert_eq!(p.thresh_crit, 88);
}

#[test]
fn warn_crit_override_defaults() {
    let cli = Cli::parse_from(["storageshower", "-w", "0", "-C", "100"]);
    let mut p = Prefs {
        thresh_warn: 70,
        thresh_crit: 90,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 0);
    assert_eq!(p.thresh_crit, 100);
}
