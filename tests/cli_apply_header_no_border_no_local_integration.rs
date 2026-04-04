//! `Cli::apply_to` for `--header`, `--no-border`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn header_no_border_no_local() {
    let cli = Cli::parse_from(["storageshower", "--header", "--no-border", "--no-local"]);
    let mut p = Prefs {
        show_header: false,
        show_border: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_header);
    assert!(!p.show_border);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_header_and_border() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_header: true,
        show_border: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.show_header);
    assert!(!p.show_border);
}
