//! TOML `bookmarks` with seven paths.

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
fn deserialize_seven_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/q1", "/q2", "/q3", "/q4", "/q5", "/q6", "/q7"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 7);
    assert_eq!(p.bookmarks[0], "/q1");
    assert_eq!(p.bookmarks[6], "/q7");
}

#[test]
fn roundtrip_seven_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (1..=7).map(|i| format!("/m{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
