//! `Cli::apply_to` for `--no-compact`, `--no-virtual`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_compact_no_virtual_no_local() {
    let cli = Cli::parse_from([
        "storageshower",
        "--no-compact",
        "--no-virtual",
        "--no-local",
    ]);
    let mut p = Prefs {
        compact: true,
        show_all: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.compact);
    assert!(!p.show_all);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_compact_and_virtual() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        compact: false,
        show_all: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(!p.compact);
    assert!(p.show_all);
}
