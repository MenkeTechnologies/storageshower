//! `Cli::apply_to` for `--used`, `--header`, and `--tooltips` when prefs start disabled.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn enables_used_header_tooltips() {
    let cli = Cli::parse_from(["storageshower", "--used", "--header", "--tooltips"]);
    let mut p = Prefs {
        show_used: false,
        show_header: false,
        show_tooltips: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_used);
    assert!(p.show_header);
    assert!(p.show_tooltips);
}

#[test]
fn tooltips_only_from_all_false() {
    let cli = Cli::parse_from(["storageshower", "--tooltips"]);
    let mut p = Prefs {
        show_tooltips: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_tooltips);
}
