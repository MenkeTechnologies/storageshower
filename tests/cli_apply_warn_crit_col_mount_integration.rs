//! `Cli::apply_to` for `-w`, `-C`, and `--col-mount`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_col_mount() {
    let cli = Cli::parse_from(["storageshower", "-w", "55", "-C", "91", "--col-mount", "28"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 55);
    assert_eq!(p.thresh_crit, 91);
    assert_eq!(p.col_mount_w, 28);
}

#[test]
fn warn_only_col_mount_preserves_crit() {
    let cli = Cli::parse_from(["storageshower", "-w", "22", "--col-mount", "10"]);
    let mut p = Prefs {
        thresh_crit: 95,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 22);
    assert_eq!(p.col_mount_w, 10);
    assert_eq!(p.thresh_crit, 95);
}
