//! `Cli::apply_to` for `-k` (compact) and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn compact_no_local() {
    let cli = Cli::parse_from(["storageshower", "-k", "--no-local"]);
    let mut p = Prefs {
        compact: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.compact);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        compact: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.compact);
}
