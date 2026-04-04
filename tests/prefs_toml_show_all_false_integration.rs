//! TOML `show_all = false` for `Prefs`.

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

#[test]
fn deserialize_show_all_false() {
    let s = format!(
        r#"
{BASE}
show_all = false
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert!(!p.show_all);
}

#[test]
fn roundtrip_show_all_false() {
    let mut p = Prefs::default();
    p.show_all = false;
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert!(!q.show_all);
}
