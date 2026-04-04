//! `Cli::apply_to` for `--refresh`, `--warn`, and `--crit` together.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_and_thresholds() {
    let cli = Cli::parse_from([
        "storageshower",
        "--refresh",
        "15",
        "--warn",
        "33",
        "--crit",
        "77",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 15);
    assert_eq!(p.thresh_warn, 33);
    assert_eq!(p.thresh_crit, 77);
}

#[test]
fn short_flags_r_w_c() {
    let cli = Cli::parse_from(["storageshower", "-r", "3", "-w", "10", "-C", "99"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 3);
    assert_eq!(p.thresh_warn, 10);
    assert_eq!(p.thresh_crit, 99);
}
