//! `Cli::apply_to` for `--color`, `--theme`, and `--compact`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::ColorMode;

#[test]
fn color_theme_compact() {
    let cli = Cli::parse_from([
        "storageshower",
        "--color",
        "purple",
        "--theme",
        "neon_slot",
        "--compact",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Purple);
    assert_eq!(p.active_theme.as_deref(), Some("neon_slot"));
    assert!(p.compact);
}

#[test]
fn color_only_sets_palette() {
    let cli = Cli::parse_from(["storageshower", "--color", "cyan"]);
    let mut p = Prefs {
        color_mode: ColorMode::Red,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Cyan);
}
