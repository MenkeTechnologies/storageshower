//! `Cli::apply_to` for `--no-border`, `--no-header`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_border_no_header_no_local() {
    let cli = Cli::parse_from(["storageshower", "--no-border", "--no-header", "--no-local"]);
    let mut p = Prefs {
        show_border: true,
        show_header: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_border);
    assert!(!p.show_header);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_border_and_header() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_border: true,
        show_header: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.show_border);
    assert!(!p.show_header);
}
