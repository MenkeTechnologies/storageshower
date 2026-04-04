//! `Cli::apply_to`: `--header` / `--no-header` — last flag wins (clap `overrides_with`).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_header_then_header_enables() {
    let cli = Cli::parse_from(["storageshower", "--no-header", "--header"]);
    let mut p = Prefs {
        show_header: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_header);
}

#[test]
fn header_then_no_header_disables() {
    let cli = Cli::parse_from(["storageshower", "--header", "--no-header"]);
    let mut p = Prefs {
        show_header: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_header);
}
