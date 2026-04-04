//! `Cli::apply_to` for `-w`, `-C`, and `--sort pct`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn warn_crit_sort_pct() {
    let cli = Cli::parse_from(["storageshower", "-w", "63", "-C", "97", "--sort", "pct"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 63);
    assert_eq!(p.thresh_crit, 97);
    assert_eq!(p.sort_mode, SortMode::Pct);
}

#[test]
fn warn_short_sort_pct() {
    let cli = Cli::parse_from(["storageshower", "-w", "12", "-s", "pct"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 12);
    assert_eq!(p.sort_mode, SortMode::Pct);
}
