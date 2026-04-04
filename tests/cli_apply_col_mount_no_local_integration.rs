//! `Cli::apply_to` for `--col-mount` and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn col_mount_no_local() {
    let cli = Cli::parse_from(["storageshower", "--col-mount", "22", "--no-local"]);
    let mut p = Prefs {
        col_mount_w: 0,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_mount_w, 22);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_col_mount() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        col_mount_w: 17,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.col_mount_w, 17);
}
