//! `Cli::apply_to` for `-C` (crit), `--sort`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn crit_sort_pct_no_local() {
    let cli = Cli::parse_from(["storageshower", "-C", "86", "-s", "pct", "--no-local"]);
    let mut p = Prefs {
        thresh_crit: 90,
        sort_mode: SortMode::Name,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_crit, 86);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_crit_and_sort() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        thresh_crit: 88,
        sort_mode: SortMode::Size,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.thresh_crit, 88);
    assert_eq!(p.sort_mode, SortMode::Size);
}
