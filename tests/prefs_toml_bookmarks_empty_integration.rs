//! TOML `bookmarks = []` and roundtrip.

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
fn deserialize_bookmarks_empty_array() {
    let s = format!(
        r#"
{BASE}
bookmarks = []
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert!(p.bookmarks.is_empty());
}

#[test]
fn roundtrip_empty_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks.clear();
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert!(q.bookmarks.is_empty());
}
