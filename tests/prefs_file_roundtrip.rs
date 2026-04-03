//! `load_prefs_from(Some(path))` with real temp files (integration path for prefs I/O).
#![allow(clippy::field_reassign_with_default)]

use std::collections::HashMap;
use std::fs;

use tempfile::tempdir;

use storageshower::prefs::{Prefs, load_prefs_from};
use storageshower::types::{BarStyle, ColorMode, SortMode, ThemeColors, UnitMode};

fn write_load(p: &Prefs) -> Prefs {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("storageshower-test.prefs.toml");
    let contents = toml::to_string_pretty(p).expect("serialize prefs");
    fs::write(&path, contents).expect("write prefs");
    load_prefs_from(Some(path.to_str().expect("utf8 path")))
}

#[test]
fn roundtrip_default_prefs() {
    let expected = Prefs::default();
    let loaded = write_load(&expected);
    assert_eq!(loaded.sort_mode, expected.sort_mode);
    assert_eq!(loaded.refresh_rate, expected.refresh_rate);
    assert_eq!(loaded.color_mode, expected.color_mode);
    assert_eq!(loaded.thresh_warn, expected.thresh_warn);
    assert_eq!(loaded.thresh_crit, expected.thresh_crit);
}

#[test]
fn roundtrip_sort_size_bar_ascii() {
    let mut p = Prefs::default();
    p.sort_mode = SortMode::Size;
    p.bar_style = BarStyle::Ascii;
    let loaded = write_load(&p);
    assert_eq!(loaded.sort_mode, SortMode::Size);
    assert_eq!(loaded.bar_style, BarStyle::Ascii);
}

#[test]
fn roundtrip_color_zaibatsu_units_mib() {
    let mut p = Prefs::default();
    p.color_mode = ColorMode::Zaibatsu;
    p.unit_mode = UnitMode::MiB;
    let loaded = write_load(&p);
    assert_eq!(loaded.color_mode, ColorMode::Zaibatsu);
    assert_eq!(loaded.unit_mode, UnitMode::MiB);
}

#[test]
fn roundtrip_sort_pct_rev_and_local() {
    let mut p = Prefs::default();
    p.sort_mode = SortMode::Pct;
    p.sort_rev = true;
    p.show_local = true;
    let loaded = write_load(&p);
    assert_eq!(loaded.sort_mode, SortMode::Pct);
    assert!(loaded.sort_rev);
    assert!(loaded.show_local);
}

#[test]
fn roundtrip_thresholds_and_refresh() {
    let mut p = Prefs::default();
    p.thresh_warn = 55;
    p.thresh_crit = 88;
    p.refresh_rate = 42;
    let loaded = write_load(&p);
    assert_eq!(loaded.thresh_warn, 55);
    assert_eq!(loaded.thresh_crit, 88);
    assert_eq!(loaded.refresh_rate, 42);
}

#[test]
fn roundtrip_column_widths() {
    let mut p = Prefs::default();
    p.col_mount_w = 22;
    p.col_bar_end_w = 40;
    p.col_pct_w = 9;
    let loaded = write_load(&p);
    assert_eq!(loaded.col_mount_w, 22);
    assert_eq!(loaded.col_bar_end_w, 40);
    assert_eq!(loaded.col_pct_w, 9);
}

#[test]
fn roundtrip_bookmarks() {
    let mut p = Prefs::default();
    p.bookmarks = vec!["/a".into(), "/b".into(), "/c".into()];
    let loaded = write_load(&p);
    assert_eq!(loaded.bookmarks, vec!["/a", "/b", "/c"]);
}

#[test]
fn roundtrip_active_theme() {
    let mut p = Prefs::default();
    p.active_theme = Some("my-neon".into());
    let loaded = write_load(&p);
    assert_eq!(loaded.active_theme.as_deref(), Some("my-neon"));
}

#[test]
fn roundtrip_show_all_false_and_tooltips_false() {
    let mut p = Prefs::default();
    p.show_all = false;
    p.show_tooltips = false;
    let loaded = write_load(&p);
    assert!(!loaded.show_all);
    assert!(!loaded.show_tooltips);
}

#[test]
fn roundtrip_chrome_flags_off() {
    let mut p = Prefs::default();
    p.show_bars = false;
    p.show_border = false;
    p.show_header = false;
    p.compact = true;
    p.full_mount = true;
    let loaded = write_load(&p);
    assert!(!loaded.show_bars);
    assert!(!loaded.show_border);
    assert!(!loaded.show_header);
    assert!(loaded.compact);
    assert!(loaded.full_mount);
}

#[test]
fn roundtrip_custom_theme_entry() {
    let mut p = Prefs::default();
    let mut m = HashMap::new();
    m.insert(
        "test-theme".into(),
        ThemeColors {
            blue: 1,
            green: 2,
            purple: 3,
            light_purple: 4,
            royal: 5,
            dark_purple: 6,
        },
    );
    p.custom_themes = m;
    let loaded = write_load(&p);
    let th = loaded.custom_themes.get("test-theme").expect("theme");
    assert_eq!(th.blue, 1);
    assert_eq!(th.dark_purple, 6);
}

#[test]
fn roundtrip_two_custom_themes() {
    let mut p = Prefs::default();
    p.custom_themes.insert(
        "a".into(),
        ThemeColors {
            blue: 10,
            green: 20,
            purple: 30,
            light_purple: 40,
            royal: 50,
            dark_purple: 60,
        },
    );
    p.custom_themes.insert(
        "b".into(),
        ThemeColors {
            blue: 255,
            green: 0,
            purple: 128,
            light_purple: 64,
            royal: 32,
            dark_purple: 16,
        },
    );
    let loaded = write_load(&p);
    assert_eq!(loaded.custom_themes.len(), 2);
    assert_eq!(loaded.custom_themes.get("a").unwrap().green, 20);
    assert_eq!(loaded.custom_themes.get("b").unwrap().blue, 255);
}

#[test]
fn roundtrip_color_blade_runner_unit_gib() {
    let mut p = Prefs::default();
    p.color_mode = ColorMode::BladeRunner;
    p.unit_mode = UnitMode::GiB;
    let loaded = write_load(&p);
    assert_eq!(loaded.color_mode, ColorMode::BladeRunner);
    assert_eq!(loaded.unit_mode, UnitMode::GiB);
}

#[test]
fn roundtrip_bar_style_thin() {
    let mut p = Prefs::default();
    p.bar_style = BarStyle::Thin;
    let loaded = write_load(&p);
    assert_eq!(loaded.bar_style, BarStyle::Thin);
}

#[test]
fn roundtrip_show_used_false() {
    let mut p = Prefs::default();
    p.show_used = false;
    let loaded = write_load(&p);
    assert!(!loaded.show_used);
}

#[test]
fn load_prefs_from_missing_file_is_defaults() {
    let p = load_prefs_from(Some("/tmp/does-not-exist-storageshower-xyz-99999.conf"));
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.refresh_rate, 1);
}
