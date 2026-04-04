//! `Cli::apply_to` for `--theme` with `--virtual` and `--tooltips`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn theme_virtual_tooltips() {
    let cli = Cli::parse_from([
        "storageshower",
        "--theme",
        "neon_bundle",
        "--virtual",
        "--tooltips",
    ]);
    let mut p = Prefs {
        show_all: false,
        show_tooltips: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("neon_bundle"));
    assert!(p.show_all);
    assert!(p.show_tooltips);
}

#[test]
fn theme_no_tooltips_after_true_prefs() {
    let cli = Cli::parse_from(["storageshower", "--theme", "x", "--no-tooltips"]);
    let mut p = Prefs {
        show_tooltips: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("x"));
    assert!(!p.show_tooltips);
}
