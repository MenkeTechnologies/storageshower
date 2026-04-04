//! TOML `bookmarks` with eighty-one paths.

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
fn deserialize_eighty_one_bookmarks() {
    let entries = (0..81)
        .map(|i| format!("\"/w{i}\""))
        .collect::<Vec<_>>()
        .join(",");
    let s = format!(
        r#"
{base}
bookmarks = [{entries}]
"#,
        base = BASE,
        entries = entries
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks.len(), 81);
    assert_eq!(p.bookmarks[0], "/w0");
    assert_eq!(p.bookmarks[80], "/w80");
}

#[test]
fn roundtrip_eighty_one_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = (0..81).map(|i| format!("/bm{i}")).collect();
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, p.bookmarks);
}
