//! TOML deserialize for `Prefs` boolean / display flags not covered elsewhere.
#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

fn minimal_toml(
    sort_rev: bool,
    show_bars: bool,
    show_border: bool,
    show_header: bool,
    compact: bool,
    full_mount: bool,
    refresh_rate: u64,
) -> String {
    format!(
        r#"
sort_mode = "Name"
sort_rev = {sort_rev}
show_local = false
refresh_rate = {refresh_rate}
bar_style = "Gradient"
color_mode = "Default"
thresh_warn = 70
thresh_crit = 90
show_bars = {show_bars}
show_border = {show_border}
show_header = {show_header}
compact = {compact}
show_used = true
full_mount = {full_mount}
"#
    )
}

#[test]
fn deserialize_show_header_false() {
    let t = minimal_toml(false, true, true, false, false, false, 1);
    let p: Prefs = toml::from_str(&t).unwrap();
    assert!(!p.show_header);
}

#[test]
fn deserialize_show_bars_false() {
    let t = minimal_toml(false, false, true, true, false, false, 1);
    let p: Prefs = toml::from_str(&t).unwrap();
    assert!(!p.show_bars);
}

#[test]
fn deserialize_show_border_false() {
    let t = minimal_toml(false, true, false, true, false, false, 1);
    let p: Prefs = toml::from_str(&t).unwrap();
    assert!(!p.show_border);
}

#[test]
fn deserialize_compact_true() {
    let t = minimal_toml(false, true, true, true, true, false, 1);
    let p: Prefs = toml::from_str(&t).unwrap();
    assert!(p.compact);
}

#[test]
fn deserialize_full_mount_true() {
    let t = minimal_toml(false, true, true, true, false, true, 1);
    let p: Prefs = toml::from_str(&t).unwrap();
    assert!(p.full_mount);
}

#[test]
fn deserialize_sort_rev_true_with_name() {
    let t = minimal_toml(true, true, true, true, false, false, 1);
    let p: Prefs = toml::from_str(&t).unwrap();
    assert!(p.sort_rev);
    assert_eq!(p.sort_mode, SortMode::Name);
}

#[test]
fn deserialize_refresh_rate_42() {
    let t = minimal_toml(false, true, true, true, false, false, 42);
    let p: Prefs = toml::from_str(&t).unwrap();
    assert_eq!(p.refresh_rate, 42);
}

#[test]
fn roundtrip_prefs_with_false_flags_via_toml() {
    let mut p = Prefs::default();
    p.show_bars = false;
    p.show_header = false;
    p.compact = true;
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert!(!q.show_bars);
    assert!(!q.show_header);
    assert!(q.compact);
}
