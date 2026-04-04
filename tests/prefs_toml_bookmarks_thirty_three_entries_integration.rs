//! TOML `bookmarks` with thirty-three paths.

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
fn deserialize_thirty_three_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/w0", "/w1", "/w2", "/w3", "/w4", "/w5", "/w6", "/w7", "/w8", "/w9", "/w10", "/w11", "/w12", "/w13", "/w14", "/w15", "/w16", "/w17", "/w18", "/w19", "/w20", "/w21", "/w22", "/w23", "/w24", "/w25", "/w26", "/w27", "/w28", "/w29", "/w30", "/w31", "/w32"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 33);
    assert_eq!(p.bookmarks[0], "/w0");
    assert_eq!(p.bookmarks[32], "/w32");
}

#[test]
fn roundtrip_thirty_three_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..33).map(|i| format!("/bm{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
