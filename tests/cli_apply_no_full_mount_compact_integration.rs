//! `Cli::apply_to` for `--no-full-mount` with `-k` / `--compact`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_full_mount_and_compact() {
    let cli = Cli::parse_from(["storageshower", "--no-full-mount", "-k"]);
    let mut p = Prefs {
        full_mount: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.full_mount);
    assert!(p.compact);
}

#[test]
fn compact_then_no_full_mount_last_wins_on_full() {
    let cli = Cli::parse_from(["storageshower", "-f", "--no-full-mount", "--compact"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(!p.full_mount);
    assert!(p.compact);
}
