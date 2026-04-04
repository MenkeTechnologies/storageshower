//! `Cli` parse + `apply_to` for `--theme`, `--export-theme`, and `--color`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::ColorMode;

#[test]
fn export_theme_color_parse() {
    let cli = Cli::parse_from([
        "storageshower",
        "--export-theme",
        "--color",
        "matrix",
        "--theme",
        "export_slot",
    ]);
    assert!(cli.export_theme);
    assert_eq!(cli.color_mode, Some(ColorMode::Matrix));
    assert_eq!(cli.theme.as_deref(), Some("export_slot"));
}

#[test]
fn export_theme_color_apply_to_prefs() {
    let cli = Cli::parse_from(["storageshower", "--export-theme", "--color", "sunset"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Sunset);
}
