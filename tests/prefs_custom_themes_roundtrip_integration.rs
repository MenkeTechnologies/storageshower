//! Integration tests for `Prefs` TOML persistence: multi-theme maps, bookmarks, and roundtrips.
#![allow(clippy::field_reassign_with_default)]

use std::collections::HashMap;

use storageshower::prefs::{Prefs, load_prefs_from};
use storageshower::types::{BarStyle, ColorMode, SortMode, ThemeColors, UnitMode};

/// Minimal valid prefs stanza (matches `prefs.rs` deserialize expectations).
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

fn theme(a: u8) -> ThemeColors {
    ThemeColors {
        blue: a,
        green: a.wrapping_add(1),
        purple: a.wrapping_add(2),
        light_purple: a.wrapping_add(3),
        royal: a.wrapping_add(4),
        dark_purple: a.wrapping_add(5),
    }
}

#[test]
fn load_prefs_from_file_two_custom_theme_keys() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("prefs.toml");
    let body = format!(
        r#"{BASE}

[custom_themes.slot-a]
blue = 10
green = 11
purple = 12
light_purple = 13
royal = 14
dark_purple = 15

[custom_themes.slot_b]
blue = 20
green = 21
purple = 22
light_purple = 23
royal = 24
dark_purple = 25
"#
    );
    std::fs::write(&path, body).expect("write");
    let p = load_prefs_from(Some(path.to_str().expect("utf8")));
    assert_eq!(p.custom_themes.len(), 2);
    assert_eq!(p.custom_themes.get("slot-a").expect("slot-a").blue, 10);
    assert_eq!(p.custom_themes.get("slot_b").expect("slot_b").blue, 20);
}

#[test]
fn load_prefs_from_unicode_bookmarks() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("prefs.toml");
    let body = format!(
        r#"{BASE}
bookmarks = ["/home/用户", "/Volumes/名前"]
"#
    );
    std::fs::write(&path, body).expect("write");
    let p = load_prefs_from(Some(path.to_str().expect("utf8")));
    assert_eq!(p.bookmarks.len(), 2);
    assert_eq!(p.bookmarks[0], "/home/用户");
    assert_eq!(p.bookmarks[1], "/Volumes/名前");
}

#[test]
fn toml_roundtrip_preserves_custom_themes_and_active_theme() {
    let mut p = Prefs::default();
    p.sort_mode = SortMode::Pct;
    p.color_mode = ColorMode::Cyan;
    p.unit_mode = UnitMode::MiB;
    p.active_theme = Some("my-preset".into());
    p.custom_themes.insert("a".into(), theme(1));
    p.custom_themes.insert("b".into(), theme(50));

    let s = toml::to_string_pretty(&p).expect("serialize");
    let q: Prefs = toml::from_str(&s).expect("deserialize");
    assert_eq!(q.sort_mode, SortMode::Pct);
    assert_eq!(q.color_mode, ColorMode::Cyan);
    assert_eq!(q.unit_mode, UnitMode::MiB);
    assert_eq!(q.active_theme.as_deref(), Some("my-preset"));
    assert_eq!(q.custom_themes.len(), 2);
    assert_eq!(q.custom_themes.get("a").expect("a").blue, 1);
    assert_eq!(q.custom_themes.get("b").expect("b").royal, 54);
}

#[test]
fn toml_roundtrip_bookmarks_order_preserved() {
    let mut p = Prefs::default();
    p.bookmarks = vec!["/z".into(), "/a".into(), "/m".into()];
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.bookmarks, vec!["/z", "/a", "/m"]);
}

#[test]
fn prefs_default_custom_themes_empty() {
    let p = Prefs::default();
    assert!(p.custom_themes.is_empty());
}

#[test]
fn deserialize_explicit_false_show_all() {
    let t = format!(
        r#"{BASE}
show_all = false
"#
    );
    let p: Prefs = toml::from_str(&t).unwrap();
    assert!(!p.show_all);
}

#[test]
fn deserialize_unit_mode_mib_toml_variant() {
    // TOML uses serde enum names (`MiB`); CLI accepts the `mib` alias via clap.
    let t = format!(
        r#"{BASE}
unit_mode = "MiB"
"#
    );
    let p: Prefs = toml::from_str(&t).unwrap();
    assert_eq!(p.unit_mode, UnitMode::MiB);
}

#[test]
fn deserialize_bar_style_thin_and_sort_size() {
    let t = r#"
sort_mode = "Size"
sort_rev = false
show_local = false
refresh_rate = 1
bar_style = "Thin"
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
    let p: Prefs = toml::from_str(t).unwrap();
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.bar_style, BarStyle::Thin);
}

#[test]
fn roundtrip_hashmap_single_entry_via_toml() {
    let mut m = HashMap::new();
    m.insert("only".into(), theme(99));
    let mut p = Prefs::default();
    p.custom_themes = m;
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.custom_themes.len(), 1);
    assert_eq!(q.custom_themes.get("only").unwrap().green, 100);
}

#[test]
fn load_prefs_from_empty_body_uses_defaults() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("empty.toml");
    std::fs::write(&path, "").expect("write");
    let p = load_prefs_from(Some(path.to_str().expect("utf8")));
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.refresh_rate, 1);
}

#[test]
fn theme_colors_channel_boundaries_roundtrip_in_custom_themes() {
    let mut p = Prefs::default();
    p.custom_themes.insert(
        "edge".into(),
        ThemeColors {
            blue: 0,
            green: 255,
            purple: 128,
            light_purple: 1,
            royal: 254,
            dark_purple: 127,
        },
    );
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    let t = q.custom_themes.get("edge").expect("edge");
    assert_eq!(t.green, 255);
    assert_eq!(t.blue, 0);
}

#[test]
fn deserialize_col_width_prefs() {
    let t = format!(
        r#"{BASE}
col_mount_w = 42
col_bar_end_w = 48
col_pct_w = 9
"#
    );
    let p: Prefs = toml::from_str(&t).unwrap();
    assert_eq!(p.col_mount_w, 42);
    assert_eq!(p.col_bar_end_w, 48);
    assert_eq!(p.col_pct_w, 9);
}

#[test]
fn load_prefs_active_theme_and_bookmarks_from_file_together() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("prefs.toml");
    let body = format!(
        r#"{BASE}
active_theme = "neon"
bookmarks = ["/", "/srv"]

[custom_themes.neon]
blue = 1
green = 2
purple = 3
light_purple = 4
royal = 5
dark_purple = 6
"#
    );
    std::fs::write(&path, body).expect("write");
    let p = load_prefs_from(Some(path.to_str().expect("utf8")));
    assert_eq!(p.active_theme.as_deref(), Some("neon"));
    assert_eq!(p.bookmarks, vec!["/", "/srv"]);
    assert!(p.custom_themes.contains_key("neon"));
}

#[test]
fn toml_roundtrip_show_local_and_full_mount_flags() {
    let mut p = Prefs::default();
    p.show_local = true;
    p.full_mount = true;
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert!(q.show_local);
    assert!(q.full_mount);
}
