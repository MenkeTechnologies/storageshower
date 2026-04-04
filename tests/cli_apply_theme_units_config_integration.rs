//! `Cli` parse + `apply_to` for `--theme`, `--units`, and `--config`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::UnitMode;

#[test]
fn theme_and_units_mib() {
    let cli = Cli::parse_from(["storageshower", "--theme", "my_preset", "--units", "mib"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("my_preset"));
    assert_eq!(p.unit_mode, UnitMode::MiB);
}

#[test]
fn config_path_parsed_with_theme() {
    let cli = Cli::parse_from([
        "storageshower",
        "--config",
        "/tmp/storageshower-test.conf",
        "--theme",
        "exported",
    ]);
    assert_eq!(cli.config.as_deref(), Some("/tmp/storageshower-test.conf"));
    assert_eq!(cli.theme.as_deref(), Some("exported"));
}

#[test]
fn theme_gib_units() {
    let cli = Cli::parse_from(["storageshower", "--theme", "x", "-u", "gib"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::GiB);
}
