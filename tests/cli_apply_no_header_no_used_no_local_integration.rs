//! `Cli::apply_to` for `--no-header`, `--no-used`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_header_no_used_no_local() {
    let cli = Cli::parse_from(["storageshower", "--no-header", "--no-used", "--no-local"]);
    let mut p = Prefs {
        show_header: true,
        show_used: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_header);
    assert!(!p.show_used);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_header_and_used() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_header: false,
        show_used: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(!p.show_header);
    assert!(p.show_used);
}
