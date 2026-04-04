//! TOML `bookmarks` with twelve paths.

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
fn deserialize_twelve_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/v0", "/v1", "/v2", "/v3", "/v4", "/v5", "/v6", "/v7", "/v8", "/v9", "/v10", "/v11"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 12);
    assert_eq!(p.bookmarks[0], "/v0");
    assert_eq!(p.bookmarks[11], "/v11");
}

#[test]
fn roundtrip_twelve_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..12).map(|i| format!("/b{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
