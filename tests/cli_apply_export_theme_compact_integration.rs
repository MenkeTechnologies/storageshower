//! `Cli` parse + `apply_to` for `--export-theme` with `--compact`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn export_theme_and_compact_parse() {
    let cli = Cli::parse_from(["storageshower", "--export-theme", "--compact"]);
    assert!(cli.export_theme);
    assert!(cli.compact);
}

#[test]
fn export_theme_compact_apply_to_prefs() {
    let cli = Cli::parse_from(["storageshower", "--export-theme", "-k"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(p.compact);
}
