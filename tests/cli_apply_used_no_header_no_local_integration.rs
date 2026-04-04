//! `Cli::apply_to` for `--used`, `--no-header`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn used_no_header_no_local() {
    let cli = Cli::parse_from(["storageshower", "--used", "--no-header", "--no-local"]);
    let mut p = Prefs {
        show_used: false,
        show_header: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_used);
    assert!(!p.show_header);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_used_and_header() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_used: true,
        show_header: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.show_used);
    assert!(!p.show_header);
}
