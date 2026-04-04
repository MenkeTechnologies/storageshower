//! `Cli::apply_to` for `-R` / `--reverse`, `--no-virtual`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn reverse_no_virtual_no_local() {
    let cli = Cli::parse_from(["storageshower", "-R", "--no-virtual", "--no-local"]);
    let mut p = Prefs {
        sort_rev: false,
        show_all: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.sort_rev);
    assert!(!p.show_all);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_reverse_and_virtual() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        sort_rev: true,
        show_all: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.sort_rev);
    assert!(!p.show_all);
}
