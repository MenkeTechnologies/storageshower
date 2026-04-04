//! TOML `bookmarks` with eight paths.

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
fn deserialize_eight_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/b0", "/b1", "/b2", "/b3", "/b4", "/b5", "/b6", "/b7"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 8);
    assert_eq!(p.bookmarks[0], "/b0");
    assert_eq!(p.bookmarks[7], "/b7");
}

#[test]
fn roundtrip_eight_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..8).map(|i| format!("/slot{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
