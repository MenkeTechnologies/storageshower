//! `Cli::apply_to` for `-w`, `-C`, and `--no-tooltips`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_no_tooltips() {
    let cli = Cli::parse_from(["storageshower", "-w", "48", "-C", "83", "--no-tooltips"]);
    let mut p = Prefs {
        show_tooltips: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 48);
    assert_eq!(p.thresh_crit, 83);
    assert!(!p.show_tooltips);
}

#[test]
fn no_tooltips_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--no-tooltips"]);
    let mut p = Prefs {
        thresh_warn: 65,
        thresh_crit: 87,
        show_tooltips: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_tooltips);
    assert_eq!(p.thresh_warn, 65);
    assert_eq!(p.thresh_crit, 87);
}
