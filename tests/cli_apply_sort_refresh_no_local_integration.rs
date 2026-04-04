//! `Cli::apply_to` for `--sort`, `-r` (refresh), and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_pct_refresh_no_local() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "-r", "11", "--no-local"]);
    let mut p = Prefs {
        sort_mode: SortMode::Name,
        refresh_rate: 3,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.refresh_rate, 11);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_sort_and_refresh() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        sort_mode: SortMode::Size,
        refresh_rate: 8,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.refresh_rate, 8);
}
