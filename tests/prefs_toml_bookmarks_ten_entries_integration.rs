//! TOML `bookmarks` with ten paths.

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
fn deserialize_ten_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/t0", "/t1", "/t2", "/t3", "/t4", "/t5", "/t6", "/t7", "/t8", "/t9"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 10);
    assert_eq!(p.bookmarks[0], "/t0");
    assert_eq!(p.bookmarks[9], "/t9");
}

#[test]
fn roundtrip_ten_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..10).map(|i| format!("/bm{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
