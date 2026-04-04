//! `Cli::apply_to` for `--no-used` and `--no-header` together.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn hides_used_and_header() {
    let cli = Cli::parse_from(["storageshower", "--no-used", "--no-header"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(!p.show_used);
    assert!(!p.show_header);
}

#[test]
fn no_header_alone() {
    let cli = Cli::parse_from(["storageshower", "--no-header"]);
    let mut p = Prefs {
        show_header: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_header);
}
