//! `Cli::apply_to` for `-r` / `--refresh`, `--no-virtual`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_no_virtual_no_local() {
    let cli = Cli::parse_from(["storageshower", "-r", "4", "--no-virtual", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 1,
        show_all: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 4);
    assert!(!p.show_all);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_refresh_and_virtual() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 2,
        show_all: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.refresh_rate, 2);
    assert!(!p.show_all);
}
