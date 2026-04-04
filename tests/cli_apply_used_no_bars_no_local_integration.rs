//! `Cli::apply_to` for `--used`, `--no-bars`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn used_no_bars_no_local() {
    let cli = Cli::parse_from(["storageshower", "--used", "--no-bars", "--no-local"]);
    let mut p = Prefs {
        show_used: false,
        show_bars: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_used);
    assert!(!p.show_bars);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_used_and_bars() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_used: true,
        show_bars: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.show_used);
    assert!(!p.show_bars);
}
