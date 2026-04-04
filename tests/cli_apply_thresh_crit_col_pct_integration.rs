//! `Cli::apply_to` for `-w` / `-C` with `--col-pct`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_and_col_pct() {
    let cli = Cli::parse_from(["storageshower", "-w", "61", "-C", "94", "--col-pct", "13"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 61);
    assert_eq!(p.thresh_crit, 94);
    assert_eq!(p.col_pct_w, 13);
}

#[test]
fn crit_short_with_col_mount() {
    let cli = Cli::parse_from(["storageshower", "-C", "77", "--col-mount", "24"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_crit, 77);
    assert_eq!(p.col_mount_w, 24);
}
