//! `Cli::apply_to` for `-w`, `-C`, and `--sort size`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn warn_crit_sort_size() {
    let cli = Cli::parse_from(["storageshower", "-w", "44", "-C", "98", "--sort", "size"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 44);
    assert_eq!(p.thresh_crit, 98);
    assert_eq!(p.sort_mode, SortMode::Size);
}

#[test]
fn crit_short_sort_size() {
    let cli = Cli::parse_from(["storageshower", "-C", "100", "-s", "size"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_crit, 100);
    assert_eq!(p.sort_mode, SortMode::Size);
}
