//! `Cli::apply_to` for `--sort size`, `--reverse`, and `--compact`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_reverse_compact() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size", "-R", "-k"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.sort_rev);
    assert!(p.compact);
}

#[test]
fn short_sort_size_reverse_only() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--reverse"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(p.sort_rev);
}
