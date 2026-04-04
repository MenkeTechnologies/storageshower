//! `Cli::apply_to` for `-w`, `-C`, and `-f` (full mount).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_full_mount() {
    let cli = Cli::parse_from(["storageshower", "-w", "54", "-C", "92", "-f"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 54);
    assert_eq!(p.thresh_crit, 92);
    assert!(p.full_mount);
}

#[test]
fn full_mount_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "-f"]);
    let mut p = Prefs {
        thresh_warn: 61,
        thresh_crit: 89,
        full_mount: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.full_mount);
    assert_eq!(p.thresh_warn, 61);
    assert_eq!(p.thresh_crit, 89);
}
