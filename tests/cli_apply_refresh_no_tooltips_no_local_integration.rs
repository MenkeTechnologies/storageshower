//! `Cli::apply_to` for `-r` / `--refresh`, `--no-tooltips`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_no_tooltips_no_local() {
    let cli = Cli::parse_from(["storageshower", "-r", "9", "--no-tooltips", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 1,
        show_tooltips: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 9);
    assert!(!p.show_tooltips);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_refresh_and_tooltips() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 4,
        show_tooltips: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.refresh_rate, 4);
    assert!(!p.show_tooltips);
}
