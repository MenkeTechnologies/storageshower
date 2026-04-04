//! `Cli::apply_to` for `--virtual`, `--no-full-mount`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn virtual_no_full_mount_no_local() {
    let cli = Cli::parse_from([
        "storageshower",
        "--virtual",
        "--no-full-mount",
        "--no-local",
    ]);
    let mut p = Prefs {
        show_all: false,
        full_mount: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_all);
    assert!(!p.full_mount);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_virtual_and_full_mount() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_all: true,
        full_mount: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.show_all);
    assert!(!p.full_mount);
}
