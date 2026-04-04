//! TOML `bookmarks` with six paths.

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
fn deserialize_six_bookmarks() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/p1", "/p2", "/p3", "/p4", "/p5", "/p6"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(
        p.bookmarks,
        vec![
            "/p1".to_string(),
            "/p2".to_string(),
            "/p3".to_string(),
            "/p4".to_string(),
            "/p5".to_string(),
            "/p6".to_string()
        ]
    );
}

#[test]
fn roundtrip_six_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = vec![
        "/a".into(),
        "/b".into(),
        "/c".into(),
        "/d".into(),
        "/e".into(),
        "/f".into(),
    ];
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks.len(), 6);
    assert_eq!(q.bookmarks, p.bookmarks);
}
