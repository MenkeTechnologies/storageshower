//! `Cli::apply_to` for `-w` (warn), `--sort`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn warn_sort_name_no_local() {
    let cli = Cli::parse_from(["storageshower", "-w", "58", "-s", "name", "--no-local"]);
    let mut p = Prefs {
        thresh_warn: 70,
        sort_mode: SortMode::Pct,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 58);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_warn_and_sort() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        thresh_warn: 61,
        sort_mode: SortMode::Size,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.thresh_warn, 61);
    assert_eq!(p.sort_mode, SortMode::Size);
}
