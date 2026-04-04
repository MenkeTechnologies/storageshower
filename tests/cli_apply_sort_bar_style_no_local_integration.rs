//! `Cli::apply_to` for `-s size`, `-b thin`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{BarStyle, SortMode};

#[test]
fn sort_bar_style_no_local() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "-b", "thin", "--no-local"]);
    let mut p = Prefs {
        sort_mode: SortMode::Name,
        bar_style: BarStyle::Gradient,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.bar_style, BarStyle::Thin);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_sort_and_bar() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        sort_mode: SortMode::Pct,
        bar_style: BarStyle::Ascii,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.bar_style, BarStyle::Ascii);
}
