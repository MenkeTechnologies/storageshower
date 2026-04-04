//! `Cli::apply_to` for `-l` / `--local-only` and `-f` / `--full-mount`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn local_and_full_mount() {
    let cli = Cli::parse_from(["storageshower", "--local-only", "--full-mount"]);
    let mut p = Prefs {
        show_local: false,
        full_mount: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_local);
    assert!(p.full_mount);
}

#[test]
fn short_flags_l_f() {
    let cli = Cli::parse_from(["storageshower", "-l", "-f"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert!(p.show_local);
    assert!(p.full_mount);
}
