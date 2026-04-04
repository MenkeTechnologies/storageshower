//! `Cli::apply_to` for `--no-virtual` and `--no-tooltips` together.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn hides_virtual_and_tooltips_prefs() {
    let cli = Cli::parse_from(["storageshower", "--no-virtual", "--no-tooltips"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(!p.show_all);
    assert!(!p.show_tooltips);
}

#[test]
fn no_virtual_alone_from_defaults() {
    let cli = Cli::parse_from(["storageshower", "--no-virtual"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(!p.show_all);
    assert!(p.show_tooltips);
}
