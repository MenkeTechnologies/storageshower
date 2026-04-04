//! `Cli::apply_to` for `--virtual` and `--tooltips` when prefs start disabled.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn virtual_and_tooltips_enable() {
    let cli = Cli::parse_from(["storageshower", "--virtual", "--tooltips"]);
    let mut p = Prefs {
        show_all: false,
        show_tooltips: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_all);
    assert!(p.show_tooltips);
}

#[test]
fn tooltips_only() {
    let cli = Cli::parse_from(["storageshower", "--tooltips"]);
    let mut p = Prefs {
        show_tooltips: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_tooltips);
}
