//! `Cli::apply_to` for `-w`, `-C`, and `--col-bar-end`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_col_bar_end() {
    let cli = Cli::parse_from([
        "storageshower",
        "-w",
        "48",
        "-C",
        "92",
        "--col-bar-end",
        "31",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 48);
    assert_eq!(p.thresh_crit, 92);
    assert_eq!(p.col_bar_end_w, 31);
}

#[test]
fn warn_only_col_bar_end_preserves_crit() {
    let cli = Cli::parse_from(["storageshower", "-w", "19", "--col-bar-end", "4"]);
    let mut p = Prefs {
        thresh_crit: 88,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 19);
    assert_eq!(p.col_bar_end_w, 4);
    assert_eq!(p.thresh_crit, 88);
}
