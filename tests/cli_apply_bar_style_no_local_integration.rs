//! `Cli::apply_to` for `-b` (bar-style) and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::BarStyle;

#[test]
fn bar_style_thin_no_local() {
    let cli = Cli::parse_from(["storageshower", "-b", "thin", "--no-local"]);
    let mut p = Prefs {
        bar_style: BarStyle::Gradient,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Thin);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_bar_style() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        bar_style: BarStyle::Ascii,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.bar_style, BarStyle::Ascii);
}
