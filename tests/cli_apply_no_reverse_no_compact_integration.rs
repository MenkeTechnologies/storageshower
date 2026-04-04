//! `Cli::apply_to` for `--no-reverse` and `--no-compact` together.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn clears_rev_and_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-reverse", "--no-compact"]);
    let mut p = Prefs {
        sort_rev: true,
        compact: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.sort_rev);
    assert!(!p.compact);
}

#[test]
fn no_compact_alone() {
    let cli = Cli::parse_from(["storageshower", "--no-compact"]);
    let mut p = Prefs {
        compact: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.compact);
}
