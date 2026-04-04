//! `Cli` parse + `apply_to` for `--theme`, `--config`, and `--refresh`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn theme_config_refresh_parse() {
    let cli = Cli::parse_from([
        "storageshower",
        "--theme",
        "bundle",
        "--config",
        "/tmp/ss-test.conf",
        "-r",
        "7",
    ]);
    assert_eq!(cli.theme.as_deref(), Some("bundle"));
    assert_eq!(cli.config.as_deref(), Some("/tmp/ss-test.conf"));
    assert_eq!(cli.refresh_rate, Some(7));
}

#[test]
fn theme_config_refresh_apply_to_prefs() {
    let cli = Cli::parse_from([
        "storageshower",
        "-c",
        "/dev/null",
        "--theme",
        "slot",
        "--refresh",
        "3",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("slot"));
    assert_eq!(p.refresh_rate, 3);
}
