//! `Cli::apply_to` for `-w`, `-C`, and `-R` together.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_and_reverse() {
    let cli = Cli::parse_from(["storageshower", "-w", "62", "-C", "91", "-R"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 62);
    assert_eq!(p.thresh_crit, 91);
    assert!(p.sort_rev);
}

#[test]
fn long_warn_crit_reverse() {
    let cli = Cli::parse_from(["storageshower", "--warn", "5", "--crit", "99", "--reverse"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 5);
    assert_eq!(p.thresh_crit, 99);
    assert!(p.sort_rev);
}
