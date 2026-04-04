//! `Cli::apply_to` for `--tooltips` and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn tooltips_no_local() {
    let cli = Cli::parse_from(["storageshower", "--tooltips", "--no-local"]);
    let mut p = Prefs {
        show_tooltips: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_tooltips);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_show_tooltips() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_tooltips: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(!p.show_tooltips);
}
