//! `Cli::apply_to` for `--bars` and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn bars_no_local() {
    let cli = Cli::parse_from(["storageshower", "--bars", "--no-local"]);
    let mut p = Prefs {
        show_bars: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_bars);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_show_bars() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_bars: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.show_bars);
}
