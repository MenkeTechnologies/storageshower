//! `Cli::apply_to` for `-r` (refresh) and `--col-mount`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_col_mount() {
    let cli = Cli::parse_from(["storageshower", "-r", "6", "--col-mount", "32"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 6);
    assert_eq!(p.col_mount_w, 32);
}

#[test]
fn short_refresh_preserves_mount_when_absent() {
    let cli = Cli::parse_from(["storageshower", "-r", "1"]);
    let mut p = Prefs {
        col_mount_w: 19,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 1);
    assert_eq!(p.col_mount_w, 19);
}
