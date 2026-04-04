//! `Cli::apply_to` for `-w`, `-C`, `-r`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_refresh_no_local() {
    let cli = Cli::parse_from([
        "storageshower",
        "-w",
        "53",
        "-C",
        "87",
        "-r",
        "7",
        "--no-local",
    ]);
    let mut p = Prefs {
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 53);
    assert_eq!(p.thresh_crit, 87);
    assert_eq!(p.refresh_rate, 7);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_warn_crit_refresh() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        thresh_warn: 64,
        thresh_crit: 93,
        refresh_rate: 5,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.thresh_warn, 64);
    assert_eq!(p.thresh_crit, 93);
    assert_eq!(p.refresh_rate, 5);
}
