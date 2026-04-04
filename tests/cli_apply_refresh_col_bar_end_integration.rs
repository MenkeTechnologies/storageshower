//! `Cli::apply_to` for `-r` (refresh) and `--col-bar-end`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_col_bar_end() {
    let cli = Cli::parse_from(["storageshower", "-r", "9", "--col-bar-end", "14"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 9);
    assert_eq!(p.col_bar_end_w, 14);
}

#[test]
fn short_refresh_preserves_bar_end_when_absent() {
    let cli = Cli::parse_from(["storageshower", "-r", "0"]);
    let mut p = Prefs {
        col_bar_end_w: 22,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 0);
    assert_eq!(p.col_bar_end_w, 22);
}
