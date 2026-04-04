//! `Cli::apply_to` for `--theme`, `--tooltips` / `--no-tooltips`, `--virtual` / `--no-virtual`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn theme_sets_active_theme() {
    let cli = Cli::parse_from(["storageshower", "--theme", "neon_custom"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("neon_custom"));
}

#[test]
fn no_tooltips_clears_flag() {
    let cli = Cli::parse_from(["storageshower", "--no-tooltips"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(!p.show_tooltips);
}

#[test]
fn tooltips_enables_after_false_pref() {
    let cli = Cli::parse_from(["storageshower", "--tooltips"]);
    let mut p = Prefs {
        show_tooltips: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_tooltips);
}

#[test]
fn no_virtual_sets_show_all_false() {
    let cli = Cli::parse_from(["storageshower", "--no-virtual"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(!p.show_all);
}

#[test]
fn virtual_sets_show_all_true() {
    let cli = Cli::parse_from(["storageshower", "--virtual"]);
    let mut p = Prefs {
        show_all: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_all);
}

#[test]
fn theme_plus_refresh_still_applies_theme() {
    let cli = Cli::parse_from(["storageshower", "--theme", "z", "-r", "3"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("z"));
    assert_eq!(p.refresh_rate, 3);
}
