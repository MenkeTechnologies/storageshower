//! TOML with larger `bookmarks` arrays and roundtrips.
#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

fn base() -> &'static str {
    r#"
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
"#
}

#[test]
fn deserialize_ten_bookmarks() {
    let t = format!(
        r#"{}
bookmarks = ["/a", "/b", "/c", "/d", "/e", "/f", "/g", "/h", "/i", "/j"]
"#,
        base()
    );
    let p: Prefs = toml::from_str(&t).unwrap();
    assert_eq!(p.bookmarks.len(), 10);
    assert_eq!(p.bookmarks[0], "/a");
    assert_eq!(p.bookmarks[9], "/j");
}

#[test]
fn roundtrip_preserves_bookmark_order() {
    let mut p = Prefs::default();
    p.bookmarks = vec!["/z".into(), "/y".into(), "/x".into()];
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.bookmarks, vec!["/z", "/y", "/x"]);
}

#[test]
fn single_bookmark_entry() {
    let t = format!(
        r#"{}
bookmarks = ["/only"]
"#,
        base()
    );
    let p: Prefs = toml::from_str(&t).unwrap();
    assert_eq!(p.bookmarks, vec!["/only"]);
}

#[test]
fn bookmark_with_spaces_in_path() {
    let t = format!(
        r#"{}
bookmarks = ["/Volumes/My Volume"]
"#,
        base()
    );
    let p: Prefs = toml::from_str(&t).unwrap();
    assert_eq!(p.bookmarks[0], "/Volumes/My Volume");
}

#[test]
fn bookmarks_with_unicode_paths() {
    let t = format!(
        r#"{}
bookmarks = ["/home/用户", "/mnt/データ"]
"#,
        base()
    );
    let p: Prefs = toml::from_str(&t).unwrap();
    assert_eq!(p.bookmarks.len(), 2);
}
