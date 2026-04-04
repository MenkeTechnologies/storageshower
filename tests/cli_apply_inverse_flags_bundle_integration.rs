//! Multiple `--no-*` flags in one `Cli::parse` + `apply_to`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn no_bars_border_header_used_together() {
    let cli = Cli::parse_from([
        "storageshower",
        "--no-bars",
        "--no-border",
        "--no-header",
        "--no-used",
    ]);
    let mut p = Prefs {
        show_bars: true,
        show_border: true,
        show_header: true,
        show_used: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_bars);
    assert!(!p.show_border);
    assert!(!p.show_header);
    assert!(!p.show_used);
}

#[test]
fn no_compact_full_mount_tooltips() {
    let cli = Cli::parse_from([
        "storageshower",
        "--no-compact",
        "--no-full-mount",
        "--no-tooltips",
    ]);
    let mut p = Prefs {
        compact: true,
        full_mount: true,
        show_tooltips: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.compact);
    assert!(!p.full_mount);
    assert!(!p.show_tooltips);
}

#[test]
fn no_reverse_no_local_no_virtual() {
    let cli = Cli::parse_from([
        "storageshower",
        "--no-reverse",
        "--no-local",
        "--no-virtual",
    ]);
    let mut p = Prefs {
        sort_rev: true,
        show_local: true,
        show_all: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.sort_rev);
    assert!(!p.show_local);
    assert!(!p.show_all);
}
