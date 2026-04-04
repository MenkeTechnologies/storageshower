//! TOML `bookmarks` with a single entry.

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
fn deserialize_one_bookmark() {
    let s = format!(
        r#"
{BASE}
bookmarks = ["/mnt/data"]
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.bookmarks, vec!["/mnt/data".to_string()]);
}

#[test]
fn roundtrip_one_bookmark() {
    let mut p = Prefs::default();
    p.bookmarks = vec!["/only".into()];
    let out = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&out).unwrap();
    assert_eq!(q.bookmarks, vec!["/only".to_string()]);
}
