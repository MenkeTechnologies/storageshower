//! `Cli::apply_to` for `-b ascii` with `--sort pct`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{BarStyle, SortMode};

#[test]
fn ascii_bar_pct_sort() {
    let cli = Cli::parse_from(["storageshower", "-b", "ascii", "--sort", "pct"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Ascii);
    assert_eq!(p.sort_mode, SortMode::Pct);
}

#[test]
fn bar_style_ascii_short_sort_pct() {
    let cli = Cli::parse_from(["storageshower", "--bar-style", "ascii", "-s", "pct"]);
    let mut p = Prefs {
        sort_mode: SortMode::Name,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.bar_style, BarStyle::Ascii);
}
