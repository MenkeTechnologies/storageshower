//! `Cli::apply_to` for `-f` / `--full-mount`, `-l`, and `-r`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn full_mount_local_refresh() {
    let cli = Cli::parse_from(["storageshower", "-f", "-l", "-r", "13"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(p.full_mount);
    assert!(p.show_local);
    assert_eq!(p.refresh_rate, 13);
}

#[test]
fn long_flags_full_mount_no_local_refresh() {
    let cli = Cli::parse_from([
        "storageshower",
        "--full-mount",
        "--no-local",
        "--refresh",
        "1",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(p.full_mount);
    assert!(!p.show_local);
    assert_eq!(p.refresh_rate, 1);
}
