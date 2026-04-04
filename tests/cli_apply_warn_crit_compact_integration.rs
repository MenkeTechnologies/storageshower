//! `Cli::apply_to` for `-w`, `-C`, and `-k` (compact).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_compact() {
    let cli = Cli::parse_from(["storageshower", "-w", "57", "-C", "93", "-k"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 57);
    assert_eq!(p.thresh_crit, 93);
    assert!(p.compact);
}

#[test]
fn compact_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "-k"]);
    let mut p = Prefs {
        thresh_warn: 60,
        thresh_crit: 88,
        compact: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.compact);
    assert_eq!(p.thresh_warn, 60);
    assert_eq!(p.thresh_crit, 88);
}
