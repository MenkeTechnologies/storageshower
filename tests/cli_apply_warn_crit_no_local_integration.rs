//! `Cli::apply_to` for `-w`, `-C`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_no_local() {
    let cli = Cli::parse_from(["storageshower", "-w", "46", "-C", "82", "--no-local"]);
    let mut p = Prefs {
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 46);
    assert_eq!(p.thresh_crit, 82);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        thresh_warn: 68,
        thresh_crit: 91,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.thresh_warn, 68);
    assert_eq!(p.thresh_crit, 91);
}
