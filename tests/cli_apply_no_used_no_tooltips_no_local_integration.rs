//! `Cli::apply_to` for `--no-used`, `--no-tooltips`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_used_no_tooltips_no_local() {
    let cli = Cli::parse_from(["storageshower", "--no-used", "--no-tooltips", "--no-local"]);
    let mut p = Prefs {
        show_used: true,
        show_tooltips: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_used);
    assert!(!p.show_tooltips);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_used_and_tooltips() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_used: false,
        show_tooltips: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(!p.show_used);
    assert!(p.show_tooltips);
}
