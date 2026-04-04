//! `Cli::apply_to` for `-w`, `-C`, and `--no-used`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_no_used() {
    let cli = Cli::parse_from(["storageshower", "-w", "49", "-C", "85", "--no-used"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 49);
    assert_eq!(p.thresh_crit, 85);
    assert!(!p.show_used);
}

#[test]
fn no_used_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--no-used"]);
    let mut p = Prefs {
        thresh_warn: 64,
        thresh_crit: 90,
        show_used: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_used);
    assert_eq!(p.thresh_warn, 64);
    assert_eq!(p.thresh_crit, 90);
}
