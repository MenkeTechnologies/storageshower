//! TOML `bookmarks` with fifteen paths.

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
fn deserialize_fifteen_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/p0", "/p1", "/p2", "/p3", "/p4", "/p5", "/p6", "/p7", "/p8", "/p9", "/p10", "/p11", "/p12", "/p13", "/p14"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 15);
    assert_eq!(p.bookmarks[0], "/p0");
    assert_eq!(p.bookmarks[14], "/p14");
}

#[test]
fn roundtrip_fifteen_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..15).map(|i| format!("/bk{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
