//! `Cli::apply_to` for `--header`, `--no-compact`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn header_no_compact_no_local() {
    let cli = Cli::parse_from(["storageshower", "--header", "--no-compact", "--no-local"]);
    let mut p = Prefs {
        show_header: false,
        compact: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_header);
    assert!(!p.compact);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_header_and_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_header: true,
        compact: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.show_header);
    assert!(!p.compact);
}
