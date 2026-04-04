//! `Cli::apply_to` for `--sort size`, `--units`, and `--reverse`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_size_mib_reverse() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size", "-u", "mib", "-R"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.unit_mode, UnitMode::MiB);
    assert!(p.sort_rev);
}

#[test]
fn short_sort_size_gib_preserves_sort_rev_when_no_reverse_flags() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "-u", "gib"]);
    let mut p = Prefs {
        sort_rev: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.unit_mode, UnitMode::GiB);
    assert!(p.sort_rev);
}
