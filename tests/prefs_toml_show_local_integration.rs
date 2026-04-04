//! TOML `show_local` for `Prefs`.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

const BASE: &str = r#"
sort_mode = "Name"
sort_rev = false
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
fn deserialize_show_local_true() {
    let s = format!(
        r#"
{BASE}
show_local = true
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert!(p.show_local);
}

#[test]
fn roundtrip_show_local_true() {
    let mut p = Prefs::default();
    p.show_local = true;
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert!(q.show_local);
}
