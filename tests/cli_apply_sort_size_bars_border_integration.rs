//! `Cli::apply_to` for `--sort size` with `--bars` and `--border`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_bars_border_enable() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size", "--bars", "--border"]);
    let mut p = Prefs {
        show_bars: false,
        show_border: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.show_bars);
    assert!(p.show_border);
}

#[test]
fn short_sort_size_bars_only() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--bars"]);
    let mut p = Prefs {
        show_bars: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.show_bars);
}
