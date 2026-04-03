//! Single `--no-*` / inverse CLI flags applied to prefs that start in the opposite state.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

fn parse(args: &[&str]) -> Cli {
    let mut v = vec!["storageshower"];
    v.extend_from_slice(args);
    Cli::try_parse_from(v).unwrap_or_else(|e| panic!("parse {:?}: {e}", args))
}

#[test]
fn no_reverse_clears_prefs_sort_rev() {
    let c = parse(&["--no-reverse"]);
    let mut p = Prefs {
        sort_rev: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.sort_rev);
}

#[test]
fn no_local_clears_prefs_show_local() {
    let c = parse(&["--no-local"]);
    let mut p = Prefs {
        show_local: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.show_local);
}

#[test]
fn no_compact_clears_prefs_compact() {
    let c = parse(&["--no-compact"]);
    let mut p = Prefs {
        compact: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.compact);
}

#[test]
fn no_full_mount_clears_prefs_full_mount() {
    let c = parse(&["--no-full-mount"]);
    let mut p = Prefs {
        full_mount: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.full_mount);
}

#[test]
fn no_bars_clears_prefs_show_bars() {
    let c = parse(&["--no-bars"]);
    let mut p = Prefs {
        show_bars: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.show_bars);
}

#[test]
fn no_border_clears_prefs_show_border() {
    let c = parse(&["--no-border"]);
    let mut p = Prefs {
        show_border: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.show_border);
}

#[test]
fn no_header_clears_prefs_show_header() {
    let c = parse(&["--no-header"]);
    let mut p = Prefs {
        show_header: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.show_header);
}

#[test]
fn no_used_clears_prefs_show_used() {
    let c = parse(&["--no-used"]);
    let mut p = Prefs {
        show_used: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.show_used);
}

#[test]
fn no_tooltips_clears_prefs_show_tooltips() {
    let c = parse(&["--no-tooltips"]);
    let mut p = Prefs {
        show_tooltips: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.show_tooltips);
}

#[test]
fn no_virtual_sets_show_all_false() {
    let c = parse(&["--no-virtual"]);
    let mut p = Prefs {
        show_all: true,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(!p.show_all);
}

#[test]
fn bars_enables_prefs_show_bars() {
    let c = parse(&["--bars"]);
    let mut p = Prefs {
        show_bars: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_bars);
}

#[test]
fn border_enables_prefs_show_border() {
    let c = parse(&["--border"]);
    let mut p = Prefs {
        show_border: false,
        ..Default::default()
    };
    c.apply_to(&mut p);
    assert!(p.show_border);
}
