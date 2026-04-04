//! `Cli::apply_to` combining `--sort`, `--refresh`, and `--bar-style`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{BarStyle, SortMode};

#[test]
fn sort_size_refresh_bar_solid() {
    let cli = Cli::parse_from([
        "storageshower",
        "--sort",
        "size",
        "-r",
        "4",
        "--bar-style",
        "solid",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.refresh_rate, 4);
    assert_eq!(p.bar_style, BarStyle::Solid);
}

#[test]
fn sort_pct_refresh_one_bar_thin() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "--refresh", "1", "-b", "thin"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.refresh_rate, 1);
    assert_eq!(p.bar_style, BarStyle::Thin);
}
