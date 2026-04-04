//! `Cli::apply_to` for `-r` (refresh) and `--col-pct`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_col_pct() {
    let cli = Cli::parse_from(["storageshower", "-r", "7", "--col-pct", "12"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 7);
    assert_eq!(p.col_pct_w, 12);
}

#[test]
fn short_refresh_preserves_pct_when_col_pct_absent() {
    let cli = Cli::parse_from(["storageshower", "-r", "2"]);
    let mut p = Prefs {
        col_pct_w: 8,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 2);
    assert_eq!(p.col_pct_w, 8);
}
