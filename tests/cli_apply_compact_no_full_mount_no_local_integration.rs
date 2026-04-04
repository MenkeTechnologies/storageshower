//! `Cli::apply_to` for `--compact`, `--no-full-mount`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn compact_no_full_mount_no_local() {
    let cli = Cli::parse_from([
        "storageshower",
        "--compact",
        "--no-full-mount",
        "--no-local",
    ]);
    let mut p = Prefs {
        compact: false,
        full_mount: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.compact);
    assert!(!p.full_mount);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_compact_and_full_mount() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        compact: true,
        full_mount: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.compact);
    assert!(!p.full_mount);
}
