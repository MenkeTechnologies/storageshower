//! `Cli::apply_to` for `--no-reverse` and `--no-local` together.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn clears_rev_and_local_only() {
    let cli = Cli::parse_from(["storageshower", "--no-reverse", "--no-local"]);
    let mut p = Prefs {
        sort_rev: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.sort_rev);
    assert!(!p.show_local);
}

#[test]
fn no_local_alone_from_local_true() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
}
