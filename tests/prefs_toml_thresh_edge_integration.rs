//! TOML `thresh_warn` / `thresh_crit` edge values.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

fn prefs_toml(warn: u8, crit: u8) -> String {
    format!(
        r#"
sort_mode = "Name"
sort_rev = false
show_local = false
refresh_rate = 1
bar_style = "Gradient"
color_mode = "Default"
thresh_warn = {warn}
thresh_crit = {crit}
show_bars = true
show_border = true
show_header = true
compact = false
show_used = true
full_mount = false
"#
    )
}

#[test]
fn deserialize_thresh_warn_zero() {
    let p: Prefs = toml::from_str(&prefs_toml(0, 90)).unwrap();
    assert_eq!(p.thresh_warn, 0);
    assert_eq!(p.thresh_crit, 90);
}

#[test]
fn deserialize_thresh_crit_hundred() {
    let p: Prefs = toml::from_str(&prefs_toml(50, 100)).unwrap();
    assert_eq!(p.thresh_warn, 50);
    assert_eq!(p.thresh_crit, 100);
}

#[test]
fn roundtrip_warn_crit() {
    let p: Prefs = toml::from_str(&prefs_toml(1, 99)).unwrap();
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.thresh_warn, 1);
    assert_eq!(q.thresh_crit, 99);
}
