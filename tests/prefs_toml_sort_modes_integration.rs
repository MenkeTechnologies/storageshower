//! TOML deserialize for each `SortMode` value.
#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

const BASE: &str = r#"
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

fn with_sort(mode: &str) -> String {
    format!(
        r#"
sort_mode = "{mode}"
{BASE}
"#
    )
}

#[test]
fn sort_mode_name() {
    let p: Prefs = toml::from_str(&with_sort("Name")).unwrap();
    assert_eq!(p.sort_mode, SortMode::Name);
}

#[test]
fn sort_mode_pct() {
    let p: Prefs = toml::from_str(&with_sort("Pct")).unwrap();
    assert_eq!(p.sort_mode, SortMode::Pct);
}

#[test]
fn sort_mode_size() {
    let p: Prefs = toml::from_str(&with_sort("Size")).unwrap();
    assert_eq!(p.sort_mode, SortMode::Size);
}

#[test]
fn roundtrip_each_sort_mode() {
    for mode in [SortMode::Name, SortMode::Pct, SortMode::Size] {
        let mut p = Prefs::default();
        p.sort_mode = mode;
        let s = toml::to_string_pretty(&p).unwrap();
        let q: Prefs = toml::from_str(&s).unwrap();
        assert_eq!(q.sort_mode, mode);
    }
}
