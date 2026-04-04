//! `Cli::apply_to` for `-w`, `-C`, and `--no-virtual`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_no_virtual() {
    let cli = Cli::parse_from(["storageshower", "-w", "50", "-C", "84", "--no-virtual"]);
    let mut p = Prefs {
        show_all: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 50);
    assert_eq!(p.thresh_crit, 84);
    assert!(!p.show_all);
}

#[test]
fn no_virtual_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--no-virtual"]);
    let mut p = Prefs {
        thresh_warn: 57,
        thresh_crit: 88,
        show_all: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_all);
    assert_eq!(p.thresh_warn, 57);
    assert_eq!(p.thresh_crit, 88);
}
