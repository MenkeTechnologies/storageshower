//! `Cli::apply_to` for `-w`, `-C`, and `--no-border`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_no_border() {
    let cli = Cli::parse_from(["storageshower", "-w", "47", "-C", "86", "--no-border"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 47);
    assert_eq!(p.thresh_crit, 86);
    assert!(!p.show_border);
}

#[test]
fn no_border_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--no-border"]);
    let mut p = Prefs {
        thresh_warn: 55,
        thresh_crit: 92,
        show_border: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_border);
    assert_eq!(p.thresh_warn, 55);
    assert_eq!(p.thresh_crit, 92);
}
