//! `Cli::apply_to` for `-r` / `--refresh`, `--no-border`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn refresh_no_border_no_local() {
    let cli = Cli::parse_from(["storageshower", "-r", "7", "--no-border", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 1,
        show_border: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 7);
    assert!(!p.show_border);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_refresh_and_border() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        refresh_rate: 2,
        show_border: false,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.refresh_rate, 2);
    assert!(!p.show_border);
}
