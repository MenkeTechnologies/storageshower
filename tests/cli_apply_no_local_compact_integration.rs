//! `Cli::apply_to` for `--no-local` (show all disks) with `--compact`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_local_and_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-local", "--compact"]);
    let mut p = Prefs {
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(p.compact);
}

#[test]
fn local_only_overrides_no_local_last_wins() {
    let cli = Cli::parse_from(["storageshower", "--no-local", "-l"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(p.show_local);
}
