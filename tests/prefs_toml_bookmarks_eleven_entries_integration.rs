//! TOML `bookmarks` with eleven paths.

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
fn deserialize_eleven_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/u0", "/u1", "/u2", "/u3", "/u4", "/u5", "/u6", "/u7", "/u8", "/u9", "/u10"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 11);
    assert_eq!(p.bookmarks[0], "/u0");
    assert_eq!(p.bookmarks[10], "/u10");
}

#[test]
fn roundtrip_eleven_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..11).map(|i| format!("/slot{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
