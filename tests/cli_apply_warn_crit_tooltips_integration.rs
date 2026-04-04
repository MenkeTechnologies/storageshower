//! `Cli::apply_to` for `-w`, `-C`, and `--tooltips`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_tooltips() {
    let cli = Cli::parse_from(["storageshower", "-w", "53", "-C", "90", "--tooltips"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 53);
    assert_eq!(p.thresh_crit, 90);
    assert!(p.show_tooltips);
}

#[test]
fn tooltips_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--tooltips"]);
    let mut p = Prefs {
        thresh_warn: 62,
        thresh_crit: 87,
        show_tooltips: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_tooltips);
    assert_eq!(p.thresh_warn, 62);
    assert_eq!(p.thresh_crit, 87);
}
