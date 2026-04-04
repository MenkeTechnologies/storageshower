//! `Cli::apply_to` for `--used`, `--no-compact`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn used_no_compact_no_local() {
    let cli = Cli::parse_from(["storageshower", "--used", "--no-compact", "--no-local"]);
    let mut p = Prefs {
        show_used: false,
        compact: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_used);
    assert!(!p.compact);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_used_and_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_used: true,
        compact: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.show_used);
    assert!(!p.compact);
}
