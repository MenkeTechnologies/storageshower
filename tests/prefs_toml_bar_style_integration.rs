//! TOML deserialize / roundtrip for `BarStyle`.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;
use storageshower::types::BarStyle;

const BASE: &str = r#"
sort_mode = "Name"
sort_rev = false
show_local = false
refresh_rate = 1
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

fn with_bar(style: &str) -> String {
    format!(
        r#"
{BASE}
bar_style = "{style}"
"#
    )
}

#[test]
fn bar_style_gradient() {
    let p: Prefs = toml::from_str(&with_bar("Gradient")).unwrap();
    assert_eq!(p.bar_style, BarStyle::Gradient);
}

#[test]
fn bar_style_solid() {
    let p: Prefs = toml::from_str(&with_bar("Solid")).unwrap();
    assert_eq!(p.bar_style, BarStyle::Solid);
}

#[test]
fn bar_style_thin() {
    let p: Prefs = toml::from_str(&with_bar("Thin")).unwrap();
    assert_eq!(p.bar_style, BarStyle::Thin);
}

#[test]
fn bar_style_ascii() {
    let p: Prefs = toml::from_str(&with_bar("Ascii")).unwrap();
    assert_eq!(p.bar_style, BarStyle::Ascii);
}

#[test]
fn roundtrip_each_bar_style() {
    for style in [
        BarStyle::Gradient,
        BarStyle::Solid,
        BarStyle::Thin,
        BarStyle::Ascii,
    ] {
        let mut p = Prefs::default();
        p.bar_style = style;
        let s = toml::to_string_pretty(&p).unwrap();
        let q: Prefs = toml::from_str(&s).unwrap();
        assert_eq!(q.bar_style, style);
    }
}
