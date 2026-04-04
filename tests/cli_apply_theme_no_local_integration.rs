//! `Cli::apply_to` for `--theme` and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn theme_no_local() {
    let cli = Cli::parse_from(["storageshower", "--theme", "custom_neon", "--no-local"]);
    let mut p = Prefs {
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("custom_neon"));
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_active_theme() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        active_theme: Some("kept_theme".into()),
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.active_theme.as_deref(), Some("kept_theme"));
}
