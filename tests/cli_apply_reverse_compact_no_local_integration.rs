//! `Cli::apply_to` for `-R` (reverse), `-k` (compact), and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn reverse_compact_no_local() {
    let cli = Cli::parse_from(["storageshower", "-R", "-k", "--no-local"]);
    let mut p = Prefs {
        sort_rev: false,
        compact: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.sort_rev);
    assert!(p.compact);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_reverse_and_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        sort_rev: true,
        compact: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.sort_rev);
    assert!(p.compact);
}
