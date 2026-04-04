//! TOML `bookmarks` with thirteen paths.

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
fn deserialize_thirteen_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/r0", "/r1", "/r2", "/r3", "/r4", "/r5", "/r6", "/r7", "/r8", "/r9", "/r10", "/r11", "/r12"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 13);
    assert_eq!(p.bookmarks[0], "/r0");
    assert_eq!(p.bookmarks[12], "/r12");
}

#[test]
fn roundtrip_thirteen_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..13).map(|i| format!("/bm{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
