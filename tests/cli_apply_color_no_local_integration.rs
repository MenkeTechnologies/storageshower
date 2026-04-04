//! `Cli::apply_to` for `--color` and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::ColorMode;

#[test]
fn color_purple_no_local() {
    let cli = Cli::parse_from(["storageshower", "--color", "purple", "--no-local"]);
    let mut p = Prefs {
        color_mode: ColorMode::Default,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::Purple);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_color_mode() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        color_mode: ColorMode::Cyan,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.color_mode, ColorMode::Cyan);
}
