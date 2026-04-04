//! TOML `bookmarks` with nine paths.

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
fn deserialize_nine_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/k0", "/k1", "/k2", "/k3", "/k4", "/k5", "/k6", "/k7", "/k8"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 9);
    assert_eq!(p.bookmarks[0], "/k0");
    assert_eq!(p.bookmarks[8], "/k8");
}

#[test]
fn roundtrip_nine_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..9).map(|i| format!("/mark{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
