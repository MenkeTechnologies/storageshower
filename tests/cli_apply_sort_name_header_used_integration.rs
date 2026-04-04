//! `Cli::apply_to` for `--sort name` with `--header` and `--used`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_name_header_used_enable() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "--header", "--used"]);
    let mut p = Prefs {
        show_header: false,
        show_used: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert!(p.show_header);
    assert!(p.show_used);
}

#[test]
fn short_sort_name_header_only() {
    let cli = Cli::parse_from(["storageshower", "-s", "name", "--header"]);
    let mut p = Prefs {
        show_header: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert!(p.show_header);
}
