//! `Cli::apply_to` for `--sort` and `-l` (local-only).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_pct_local_only() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "-l"]);
    let mut p = Prefs {
        sort_mode: SortMode::Name,
        show_local: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert!(p.show_local);
}

#[test]
fn local_only_only_preserves_sort_mode() {
    let cli = Cli::parse_from(["storageshower", "-l"]);
    let mut p = Prefs {
        sort_mode: SortMode::Size,
        show_local: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_local);
    assert_eq!(p.sort_mode, SortMode::Size);
}
