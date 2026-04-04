//! `Cli::apply_to` for `--col-pct` and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn col_pct_no_local() {
    let cli = Cli::parse_from(["storageshower", "--col-pct", "14", "--no-local"]);
    let mut p = Prefs {
        col_pct_w: 0,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_pct_w, 14);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_col_pct() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        col_pct_w: 9,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.col_pct_w, 9);
}
