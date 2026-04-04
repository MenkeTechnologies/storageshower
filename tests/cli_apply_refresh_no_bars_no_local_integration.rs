//! `Cli::apply_to` for `-r` / `--refresh`, `--no-bars`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_no_bars_no_local() {
    let cli = Cli::parse_from(["storageshower", "-r", "5", "--no-bars", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 1,
        show_bars: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 5);
    assert!(!p.show_bars);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_refresh_and_bars() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 2,
        show_bars: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.refresh_rate, 2);
    assert!(!p.show_bars);
}
