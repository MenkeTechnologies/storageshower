//! `Cli::apply_to` for `-r` (refresh) and `-R` (reverse).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_reverse() {
    let cli = Cli::parse_from(["storageshower", "-r", "13", "-R"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 13);
    assert!(p.sort_rev);
}

#[test]
fn reverse_only_preserves_refresh() {
    let cli = Cli::parse_from(["storageshower", "-R"]);
    let mut p = Prefs {
        refresh_rate: 5,
        sort_rev: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.sort_rev);
    assert_eq!(p.refresh_rate, 5);
}
