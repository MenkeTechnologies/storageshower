//! `Cli` parse + `apply_to` for `--list-colors` and `--export-theme`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn list_colors_and_export_theme_parse() {
    let cli = Cli::parse_from(["storageshower", "--list-colors", "--export-theme"]);
    assert!(cli.list_colors);
    assert!(cli.export_theme);
}

#[test]
fn export_theme_applies_compact_to_prefs() {
    let cli = Cli::parse_from([
        "storageshower",
        "--export-theme",
        "--compact",
        "--theme",
        "out",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(p.compact);
    assert_eq!(p.active_theme.as_deref(), Some("out"));
}
