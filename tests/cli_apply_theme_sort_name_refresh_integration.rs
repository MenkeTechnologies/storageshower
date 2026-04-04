//! `Cli::apply_to` for `--theme`, `--sort name`, and `--refresh`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn theme_name_sort_and_refresh() {
    let cli = Cli::parse_from([
        "storageshower",
        "--theme",
        "neon_slot",
        "--sort",
        "name",
        "-r",
        "9",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("neon_slot"));
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.refresh_rate, 9);
}

#[test]
fn theme_short_sort_refresh() {
    let cli = Cli::parse_from([
        "storageshower",
        "--theme",
        "x",
        "-s",
        "name",
        "--refresh",
        "2",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.active_theme.as_deref(), Some("x"));
    assert_eq!(p.refresh_rate, 2);
}
