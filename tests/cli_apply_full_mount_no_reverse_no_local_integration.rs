//! `Cli::apply_to` for `--full-mount`, `--no-reverse`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn full_mount_no_reverse_no_local() {
    let cli = Cli::parse_from([
        "storageshower",
        "--full-mount",
        "--no-reverse",
        "--no-local",
    ]);
    let mut p = Prefs {
        full_mount: false,
        sort_rev: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.full_mount);
    assert!(!p.sort_rev);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_full_mount_and_reverse() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        full_mount: true,
        sort_rev: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.full_mount);
    assert!(!p.sort_rev);
}
