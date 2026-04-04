//! `Cli::apply_to` combining `--sort`, `--warn`, and `--refresh`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_warn_crit_refresh() {
    let cli = Cli::parse_from([
        "storageshower",
        "--sort",
        "size",
        "-w",
        "55",
        "-C",
        "92",
        "-r",
        "8",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.thresh_warn, 55);
    assert_eq!(p.thresh_crit, 92);
    assert_eq!(p.refresh_rate, 8);
}

#[test]
fn sort_name_only_with_defaults() {
    let cli = Cli::parse_from(["storageshower", "-s", "name"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.refresh_rate, 1);
}
