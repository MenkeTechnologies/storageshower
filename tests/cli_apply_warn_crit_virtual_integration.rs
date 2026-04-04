//! `Cli::apply_to` for `-w`, `-C`, and `--virtual`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_virtual() {
    let cli = Cli::parse_from(["storageshower", "-w", "56", "-C", "91", "--virtual"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 56);
    assert_eq!(p.thresh_crit, 91);
    assert!(p.show_all);
}

#[test]
fn virtual_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--virtual"]);
    let mut p = Prefs {
        thresh_warn: 58,
        thresh_crit: 94,
        show_all: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_all);
    assert_eq!(p.thresh_warn, 58);
    assert_eq!(p.thresh_crit, 94);
}
