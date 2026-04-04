//! `Cli::apply_to` for `-r` (refresh) and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_no_local() {
    let cli = Cli::parse_from(["storageshower", "-r", "16", "--no-local"]);
    let mut p = Prefs {
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 16);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_refresh() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 7,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.refresh_rate, 7);
}
