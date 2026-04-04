//! `Cli::apply_to` for `-R` / `--reverse`, `--no-used`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn reverse_no_used_no_local() {
    let cli = Cli::parse_from(["storageshower", "-R", "--no-used", "--no-local"]);
    let mut p = Prefs {
        sort_rev: false,
        show_used: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.sort_rev);
    assert!(!p.show_used);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_reverse_and_used() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        sort_rev: true,
        show_used: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.sort_rev);
    assert!(!p.show_used);
}
