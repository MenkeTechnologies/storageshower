//! `Cli::apply_to` for `--border`, `--used`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn border_used_no_local() {
    let cli = Cli::parse_from(["storageshower", "--border", "--used", "--no-local"]);
    let mut p = Prefs {
        show_border: false,
        show_used: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_border);
    assert!(p.show_used);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_border_and_used() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_border: true,
        show_used: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.show_border);
    assert!(p.show_used);
}
