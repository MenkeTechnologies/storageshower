//! `Cli::apply_to` combining `--bar-style` and `--color`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{BarStyle, ColorMode};

#[test]
fn thin_and_purple() {
    let cli = Cli::parse_from(["storageshower", "--bar-style", "thin", "--color", "purple"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Thin);
    assert_eq!(p.color_mode, ColorMode::Purple);
}

#[test]
fn ascii_and_amber() {
    let cli = Cli::parse_from(["storageshower", "-b", "ascii", "--color", "amber"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Ascii);
    assert_eq!(p.color_mode, ColorMode::Amber);
}
