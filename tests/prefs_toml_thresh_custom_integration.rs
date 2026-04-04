//! TOML `thresh_warn` / `thresh_crit` with non-default values.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

const BASE: &str = r#"
sort_mode = "Name"
sort_rev = false
show_local = false
refresh_rate = 1
bar_style = "Gradient"
color_mode = "Default"
show_bars = true
show_border = true
show_header = true
compact = false
show_used = true
full_mount = false
"#;

#[test]
fn deserialize_custom_thresholds() {
    let s = format!(
        r#"
{BASE}
thresh_warn = 42
thresh_crit = 97
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.thresh_warn, 42);
    assert_eq!(p.thresh_crit, 97);
}

#[test]
fn roundtrip_custom_thresholds() {
    let mut p = Prefs::default();
    p.thresh_warn = 33;
    p.thresh_crit = 88;
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.thresh_warn, 33);
    assert_eq!(q.thresh_crit, 88);
}
