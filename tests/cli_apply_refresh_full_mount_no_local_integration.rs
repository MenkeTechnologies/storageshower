//! `Cli::apply_to` for `-r` / `--refresh`, `--full-mount`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_full_mount_no_local() {
    let cli = Cli::parse_from(["storageshower", "-r", "6", "--full-mount", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 1,
        full_mount: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 6);
    assert!(p.full_mount);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_refresh_and_full_mount() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 2,
        full_mount: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.refresh_rate, 2);
    assert!(p.full_mount);
}
