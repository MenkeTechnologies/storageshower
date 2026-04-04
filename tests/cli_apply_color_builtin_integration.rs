//! `Cli::apply_to` for non-hyphenated `ColorMode` values.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::ColorMode;

#[test]
fn color_default() {
    let cli = Cli::parse_from(["storageshower", "--color", "default"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Default);
}

#[test]
fn color_green() {
    let cli = Cli::parse_from(["storageshower", "--color", "green"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Green);
}

#[test]
fn color_matrix() {
    let cli = Cli::parse_from(["storageshower", "--color", "matrix"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Matrix);
}

#[test]
fn color_zaibatsu() {
    let cli = Cli::parse_from(["storageshower", "--color", "zaibatsu"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Zaibatsu);
}

#[test]
fn color_megacorp() {
    let cli = Cli::parse_from(["storageshower", "--color", "megacorp"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Megacorp);
}

#[test]
fn color_sunset() {
    let cli = Cli::parse_from(["storageshower", "--color", "sunset"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Sunset);
}
