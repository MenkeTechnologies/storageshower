//! TOML `compact` and `full_mount` flags.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

fn prefs_toml(compact: bool, full_mount: bool) -> String {
    format!(
        r#"
sort_mode = "Name"
sort_rev = false
show_local = false
refresh_rate = 1
bar_style = "Gradient"
color_mode = "Default"
thresh_warn = 70
thresh_crit = 90
show_bars = true
show_border = true
show_header = true
compact = {compact}
show_used = true
full_mount = {full_mount}
"#
    )
}

#[test]
fn deserialize_compact_true() {
    let p: Prefs = toml::from_str(&prefs_toml(true, false)).unwrap();
    assert!(p.compact);
    assert!(!p.full_mount);
}

#[test]
fn deserialize_full_mount_true() {
    let p: Prefs = toml::from_str(&prefs_toml(false, true)).unwrap();
    assert!(!p.compact);
    assert!(p.full_mount);
}

#[test]
fn roundtrip_both_true() {
    let p: Prefs = toml::from_str(&prefs_toml(true, true)).unwrap();
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert!(q.compact);
    assert!(q.full_mount);
}
