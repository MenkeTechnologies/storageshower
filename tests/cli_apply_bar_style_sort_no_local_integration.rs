//! `Cli::apply_to` for `-b` (bar-style), `--sort`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{BarStyle, SortMode};

#[test]
fn bar_style_ascii_sort_size_no_local() {
    let cli = Cli::parse_from(["storageshower", "-b", "ascii", "-s", "size", "--no-local"]);
    let mut p = Prefs {
        bar_style: BarStyle::Gradient,
        sort_mode: SortMode::Name,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Ascii);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_bar_style_and_sort() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        bar_style: BarStyle::Thin,
        sort_mode: SortMode::Pct,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.bar_style, BarStyle::Thin);
    assert_eq!(p.sort_mode, SortMode::Pct);
}
