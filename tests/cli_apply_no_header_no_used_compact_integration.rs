//! `Cli::apply_to` for `--no-header`, `--no-used`, and `--compact` together.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn hides_header_used_and_enables_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-header", "--no-used", "-k"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(!p.show_header);
    assert!(!p.show_used);
    assert!(p.compact);
}

#[test]
fn no_used_compact_only() {
    let cli = Cli::parse_from(["storageshower", "--no-used", "--compact"]);
    let mut p = Prefs {
        compact: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_used);
    assert!(p.compact);
}
