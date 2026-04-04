//! `Cli::apply_to` for `--sort name` with `-w` / `-C` thresholds.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "-w", "55", "-C", "92"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.thresh_warn, 55);
    assert_eq!(p.thresh_crit, 92);
}

#[test]
fn short_sort_name_thresh_defaults_preserved_elsewhere() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "-w", "1"]);
    let mut p = Prefs {
        thresh_crit: 99,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 1);
    assert_eq!(p.thresh_crit, 99);
}
