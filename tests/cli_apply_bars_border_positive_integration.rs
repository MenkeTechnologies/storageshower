//! `Cli::apply_to` for `--bars` and `--border` when prefs start false.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn bars_and_border_enable() {
    let cli = Cli::parse_from(["storageshower", "--bars", "--border"]);
    let mut p = Prefs {
        show_bars: false,
        show_border: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_bars);
    assert!(p.show_border);
}

#[test]
fn bars_only() {
    let cli = Cli::parse_from(["storageshower", "--bars"]);
    let mut p = Prefs {
        show_bars: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_bars);
}
