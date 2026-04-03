//! Short `clap` flags (`-s`, `-r`, `-b`, …) parse and apply like long flags.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::{BarStyle, ColorMode, SortMode, UnitMode};

fn parse(args: &[&str]) -> Cli {
    let mut v = vec!["storageshower"];
    v.extend_from_slice(args);
    Cli::try_parse_from(v).unwrap_or_else(|e| panic!("parse {:?}: {e}", args))
}

#[test]
fn short_s_sort_name() {
    let c = parse(&["-s", "name"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Name);
}

#[test]
fn short_r_reverse() {
    let c = parse(&["-R"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.sort_rev);
}

#[test]
fn short_l_local_only() {
    let c = parse(&["-l"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.show_local);
}

#[test]
fn short_r_refresh() {
    let c = parse(&["-r", "8"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.refresh_rate, 8);
}

#[test]
fn short_b_bar_style() {
    let c = parse(&["-b", "solid"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.bar_style, BarStyle::Solid);
}

#[test]
fn short_w_warn() {
    let c = parse(&["-w", "66"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 66);
}

#[test]
fn short_crit_capital_c_flag() {
    let c = parse(&["-C", "91"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.thresh_crit, 91);
}

#[test]
fn short_k_compact() {
    let c = parse(&["-k"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.compact);
}

#[test]
fn short_f_full_mount() {
    let c = parse(&["-f"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert!(p.full_mount);
}

#[test]
fn short_u_units() {
    let c = parse(&["-u", "bytes"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}

#[test]
fn short_c_config_path() {
    let c = parse(&["-c", "/etc/storageshower.conf"]);
    assert_eq!(c.config.as_deref(), Some("/etc/storageshower.conf"));
}

#[test]
fn short_flags_stack_sort_refresh_color() {
    let c = parse(&["-s", "pct", "-r", "4", "--color", "red"]);
    let mut p = Prefs::default();
    c.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Pct);
    assert_eq!(p.refresh_rate, 4);
    assert_eq!(p.color_mode, ColorMode::Red);
}
