//! `Cli::apply_to` for stacked short flags `-s`, `-l`, `-r`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_local_refresh() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "-l", "-r", "11"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.show_local);
    assert_eq!(p.refresh_rate, 11);
}

#[test]
fn sort_pct_reverse_refresh_short() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "-R", "-r", "2"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert!(p.sort_rev);
    assert_eq!(p.refresh_rate, 2);
}
