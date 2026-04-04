//! TOML `bookmarks` with fourteen paths.

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
fn deserialize_fourteen_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/s0", "/s1", "/s2", "/s3", "/s4", "/s5", "/s6", "/s7", "/s8", "/s9", "/s10", "/s11", "/s12", "/s13"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 14);
    assert_eq!(p.bookmarks[0], "/s0");
    assert_eq!(p.bookmarks[13], "/s13");
}

#[test]
fn roundtrip_fourteen_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..14).map(|i| format!("/mark{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
