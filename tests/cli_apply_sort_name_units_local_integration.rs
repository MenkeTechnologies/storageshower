//! `Cli::apply_to` for `--sort name`, `--units`, and `--local-only`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{SortMode, UnitMode};

#[test]
fn sort_name_gib_local_only() {
    let cli = Cli::parse_from(["storageshower", "--sort", "name", "-u", "gib", "-l"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.unit_mode, UnitMode::GiB);
    assert!(p.show_local);
}

#[test]
fn short_sort_name_mib_local() {
    let cli = Cli::parse_from([
        "storageshower",
        "-s",
        "name",
        "--units",
        "mib",
        "--local-only",
    ]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::MiB);
    assert!(p.show_local);
}
