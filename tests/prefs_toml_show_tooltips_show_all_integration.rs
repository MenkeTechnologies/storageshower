//! TOML for `show_tooltips` and `show_all` on `Prefs`.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

const BASE: &str = r#"
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
compact = false
show_used = true
full_mount = false
"#;

fn with_flags(show_tooltips: bool, show_all: bool) -> String {
    format!(
        r#"
{BASE}
show_tooltips = {show_tooltips}
show_all = {show_all}
"#
    )
}

#[test]
fn deserialize_show_tooltips_false() {
    let p: Prefs = toml::from_str(&with_flags(false, true)).unwrap();
    assert!(!p.show_tooltips);
    assert!(p.show_all);
}

#[test]
fn deserialize_show_all_false() {
    let p: Prefs = toml::from_str(&with_flags(true, false)).unwrap();
    assert!(p.show_tooltips);
    assert!(!p.show_all);
}

#[test]
fn roundtrip_both_false() {
    let s = with_flags(false, false);
    let p: Prefs = toml::from_str(&s).unwrap();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert!(!q.show_tooltips);
    assert!(!q.show_all);
}
