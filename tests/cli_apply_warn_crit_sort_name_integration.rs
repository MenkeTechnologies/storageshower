//! `Cli::apply_to` for `-w`, `-C`, and `--sort name`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn warn_crit_sort_name() {
    let cli = Cli::parse_from(["storageshower", "-w", "44", "-C", "88", "--sort", "name"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 44);
    assert_eq!(p.thresh_crit, 88);
    assert_eq!(p.sort_mode, SortMode::Name);
}

#[test]
fn warn_short_sort_name() {
    let cli = Cli::parse_from(["storageshower", "-w", "33", "-s", "name"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 33);
    assert_eq!(p.sort_mode, SortMode::Name);
}
