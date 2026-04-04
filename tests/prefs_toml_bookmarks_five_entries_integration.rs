//! TOML `bookmarks` with five paths.

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
fn deserialize_five_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/v", "/w", "/x", "/y", "/z"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(
        p.bookmarks,
        vec![
            "/v".to_string(),
            "/w".to_string(),
            "/x".to_string(),
            "/y".to_string(),
            "/z".to_string()
        ]
    );
}

#[test]
fn roundtrip_five_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = vec![
        "/a".into(),
        "/b".into(),
        "/c".into(),
        "/d".into(),
        "/e".into(),
    ];
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(
        q.bookmarks,
        vec![
            "/a".to_string(),
            "/b".to_string(),
            "/c".to_string(),
            "/d".to_string(),
            "/e".to_string()
        ]
    );
}
