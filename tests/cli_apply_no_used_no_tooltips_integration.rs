//! `Cli::apply_to` for `--no-used` and `--no-tooltips` together.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_used_and_no_tooltips() {
    let cli = Cli::parse_from(["storageshower", "--no-used", "--no-tooltips"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(!p.show_used);
    assert!(!p.show_tooltips);
}

#[test]
fn no_tooltips_alone() {
    let cli = Cli::parse_from(["storageshower", "--no-tooltips"]);
    let mut p = Prefs {
        show_tooltips: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_tooltips);
}
