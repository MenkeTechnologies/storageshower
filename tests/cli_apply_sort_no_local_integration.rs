//! `Cli::apply_to` for `--sort` and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_no_local() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--no-local"]);
    let mut p = Prefs {
        sort_mode: SortMode::Name,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert!(!p.show_local);
}

#[test]
fn sort_name_no_local_preserves_other_prefs() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "--no-local"]);
    let mut p = Prefs {
        thresh_warn: 55,
        thresh_crit: 88,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert!(!p.show_local);
    assert_eq!(p.thresh_warn, 55);
    assert_eq!(p.thresh_crit, 88);
}
