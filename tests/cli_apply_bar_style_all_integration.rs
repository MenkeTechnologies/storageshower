//! `Cli::apply_to` for each `BarStyle` (`-b` / `--bar-style`).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::BarStyle;

#[test]
fn bar_gradient() {
    let cli = Cli::parse_from(["storageshower", "--bar-style", "gradient"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Gradient);
}

#[test]
fn bar_solid() {
    let cli = Cli::parse_from(["storageshower", "-b", "solid"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Solid);
}

#[test]
fn bar_thin() {
    let cli = Cli::parse_from(["storageshower", "--bar-style", "thin"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Thin);
}

#[test]
fn bar_ascii() {
    let cli = Cli::parse_from(["storageshower", "-b", "ascii"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Ascii);
}
