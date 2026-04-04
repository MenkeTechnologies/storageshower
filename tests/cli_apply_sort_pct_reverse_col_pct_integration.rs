//! `Cli::apply_to` for `--sort pct`, `--reverse`, and `--col-pct`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_pct_reverse_col_pct() {
    let cli = Cli::parse_from(["storageshower", "--sort", "pct", "-R", "--col-pct", "15"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert!(p.sort_rev);
    assert_eq!(p.col_pct_w, 15);
}

#[test]
fn short_sort_pct_reverse_only() {
    let cli = Cli::parse_from(["storageshower", "-s", "pct", "--reverse"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert!(p.sort_rev);
}
