//! `Cli::apply_to` for `-r`, `-b`, and `-s` stacked.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{BarStyle, SortMode};

#[test]
fn refresh_thin_pct() {
    let cli = Cli::parse_from(["storageshower", "-r", "8", "-b", "thin", "--sort", "pct"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 8);
    assert_eq!(p.bar_style, BarStyle::Thin);
    assert_eq!(p.sort_mode, SortMode::Pct);
}

#[test]
fn refresh_solid_name() {
    let cli = Cli::parse_from([
        "storageshower",
        "--refresh",
        "3",
        "-b",
        "solid",
        "-s",
        "name",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 3);
    assert_eq!(p.bar_style, BarStyle::Solid);
    assert_eq!(p.sort_mode, SortMode::Name);
}
