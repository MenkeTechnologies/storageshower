//! `Cli::apply_to` for `-w`, `-C`, and `--col-pct`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_col_pct() {
    let cli = Cli::parse_from(["storageshower", "-w", "61", "-C", "96", "--col-pct", "9"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 61);
    assert_eq!(p.thresh_crit, 96);
    assert_eq!(p.col_pct_w, 9);
}

#[test]
fn crit_short_col_pct_preserves_warn() {
    let cli = Cli::parse_from(["storageshower", "-C", "84", "--col-pct", "2"]);
    let mut p = Prefs {
        thresh_warn: 50,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_crit, 84);
    assert_eq!(p.col_pct_w, 2);
    assert_eq!(p.thresh_warn, 50);
}
