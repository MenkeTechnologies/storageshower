//! `Cli::apply_to` for `-r` / `--refresh`, `--compact`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_compact_no_local() {
    let cli = Cli::parse_from(["storageshower", "-r", "8", "--compact", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 1,
        compact: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 8);
    assert!(p.compact);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_refresh_and_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 3,
        compact: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.refresh_rate, 3);
    assert!(p.compact);
}
