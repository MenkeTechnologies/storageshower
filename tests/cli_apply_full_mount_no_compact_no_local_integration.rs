//! `Cli::apply_to` for `--full-mount`, `--no-compact`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn full_mount_no_compact_no_local() {
    let cli = Cli::parse_from([
        "storageshower",
        "--full-mount",
        "--no-compact",
        "--no-local",
    ]);
    let mut p = Prefs {
        full_mount: false,
        compact: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.full_mount);
    assert!(!p.compact);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_full_mount_and_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        full_mount: true,
        compact: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.full_mount);
    assert!(!p.compact);
}
