//! `Cli::apply_to` for `--col-bar-end` and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn col_bar_end_no_local() {
    let cli = Cli::parse_from(["storageshower", "--col-bar-end", "21", "--no-local"]);
    let mut p = Prefs {
        col_bar_end_w: 0,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_bar_end_w, 21);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_col_bar_end() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        col_bar_end_w: 13,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert_eq!(p.col_bar_end_w, 13);
}
