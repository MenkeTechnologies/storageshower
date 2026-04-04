//! `Cli::apply_to` for `--no-bars` and `--no-border` together.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn hides_bars_and_border() {
    let cli = Cli::parse_from(["storageshower", "--no-bars", "--no-border"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(!p.show_bars);
    assert!(!p.show_border);
}

#[test]
fn no_bars_alone() {
    let cli = Cli::parse_from(["storageshower", "--no-bars"]);
    let mut p = Prefs {
        show_bars: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_bars);
}
