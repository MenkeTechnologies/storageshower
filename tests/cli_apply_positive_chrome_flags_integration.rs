//! `Cli::apply_to` for positive chrome flags when prefs start disabled.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn bars_enables_show_bars() {
    let cli = Cli::parse_from(["storageshower", "--bars"]);
    let mut p = Prefs {
        show_bars: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_bars);
}

#[test]
fn border_enables_show_border() {
    let cli = Cli::parse_from(["storageshower", "--border"]);
    let mut p = Prefs {
        show_border: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_border);
}

#[test]
fn header_enables_show_header() {
    let cli = Cli::parse_from(["storageshower", "--header"]);
    let mut p = Prefs {
        show_header: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_header);
}

#[test]
fn used_enables_show_used() {
    let cli = Cli::parse_from(["storageshower", "--used"]);
    let mut p = Prefs {
        show_used: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_used);
}

#[test]
fn compact_enables() {
    let cli = Cli::parse_from(["storageshower", "-k"]);
    let mut p = Prefs {
        compact: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.compact);
}

#[test]
fn full_mount_enables() {
    let cli = Cli::parse_from(["storageshower", "-f"]);
    let mut p = Prefs {
        full_mount: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.full_mount);
}

#[test]
fn local_only_enables_show_local() {
    let cli = Cli::parse_from(["storageshower", "-l"]);
    let mut p = Prefs {
        show_local: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.show_local);
}

#[test]
fn reverse_enables_sort_rev() {
    let cli = Cli::parse_from(["storageshower", "-R"]);
    let mut p = Prefs {
        sort_rev: false,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(p.sort_rev);
}
