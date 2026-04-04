//! `Cli::apply_to` for `-r` (refresh) and `-b` (bar-style).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::BarStyle;

#[test]
fn refresh_bar_style_thin() {
    let cli = Cli::parse_from(["storageshower", "-r", "4", "-b", "thin"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 4);
    assert_eq!(p.bar_style, BarStyle::Thin);
}

#[test]
fn short_bar_style_ascii_preserves_refresh() {
    let cli = Cli::parse_from(["storageshower", "-b", "ascii"]);
    let mut p = Prefs {
        refresh_rate: 3,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Ascii);
    assert_eq!(p.refresh_rate, 3);
}
