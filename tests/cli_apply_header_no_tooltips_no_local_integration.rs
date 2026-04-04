//! `Cli::apply_to` for `--no-header`, `--no-tooltips`, and `--no-local`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_header_no_tooltips_no_local() {
    let cli = Cli::parse_from([
        "storageshower",
        "--no-header",
        "--no-tooltips",
        "--no-local",
    ]);
    let mut p = Prefs {
        show_header: true,
        show_tooltips: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_header);
    assert!(!p.show_tooltips);
    assert!(!p.show_local);
}

#[test]
fn no_local_only_preserves_header_and_tooltips() {
    let cli = Cli::parse_from(["storageshower", "--no-local"]);
    let mut p = Prefs {
        show_header: false,
        show_tooltips: true,
        show_local: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_local);
    assert!(!p.show_header);
    assert!(p.show_tooltips);
}
