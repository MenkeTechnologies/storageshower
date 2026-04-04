//! `Cli` parse + `apply_to` for `--theme`, `--color`, `--export-theme`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::ColorMode;

#[test]
fn parse_export_theme_flag() {
    let cli = Cli::parse_from(["storageshower", "--export-theme"]);
    assert!(cli.export_theme);
}

#[test]
fn apply_theme_and_color_together() {
    let cli = Cli::parse_from(["storageshower", "--theme", "my_preset", "--color", "cyan"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("my_preset"));
    assert_eq!(p.color_mode, ColorMode::Cyan);
}

#[test]
fn parse_export_theme_with_theme_name() {
    let cli = Cli::parse_from(["storageshower", "--export-theme", "--theme", "exported"]);
    assert!(cli.export_theme);
    assert_eq!(cli.theme.as_deref(), Some("exported"));
}
