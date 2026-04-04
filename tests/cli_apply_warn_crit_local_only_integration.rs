//! `Cli::apply_to` for `-w`, `-C`, and `-l` (local-only).

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_local_only() {
    let cli = Cli::parse_from(["storageshower", "-w", "49", "-C", "83", "-l"]);
    let mut p = Prefs {
        show_local: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 49);
    assert_eq!(p.thresh_crit, 83);
    assert!(p.show_local);
}

#[test]
fn local_only_only_sets_show_local() {
    let cli = Cli::parse_from(["storageshower", "-l"]);
    let mut p = Prefs {
        thresh_warn: 62,
        thresh_crit: 94,
        show_local: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_local);
    assert_eq!(p.thresh_warn, 62);
    assert_eq!(p.thresh_crit, 94);
}
