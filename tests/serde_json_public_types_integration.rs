//! `serde_json` roundtrips for public `types` (integration crate boundary).

use storageshower::types::{BarStyle, ColorMode, SortMode, ThemeColors, UnitMode};

#[test]
fn sort_mode_json_roundtrip_all() {
    for mode in [SortMode::Name, SortMode::Pct, SortMode::Size] {
        let s = serde_json::to_string(&mode).expect("serialize");
        let back: SortMode = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(back, mode);
    }
}

#[test]
fn bar_style_json_roundtrip_all() {
    for style in [
        BarStyle::Gradient,
        BarStyle::Solid,
        BarStyle::Thin,
        BarStyle::Ascii,
    ] {
        let s = serde_json::to_string(&style).unwrap();
        let back: BarStyle = serde_json::from_str(&s).unwrap();
        assert_eq!(back, style);
    }
}

#[test]
fn unit_mode_json_roundtrip_all() {
    for u in [
        UnitMode::Human,
        UnitMode::GiB,
        UnitMode::MiB,
        UnitMode::Bytes,
    ] {
        let s = serde_json::to_string(&u).unwrap();
        let back: UnitMode = serde_json::from_str(&s).unwrap();
        assert_eq!(back, u);
    }
}

#[test]
fn color_mode_json_roundtrip_all_variants() {
    for &mode in ColorMode::ALL {
        let s = serde_json::to_string(&mode).expect("serialize");
        let back: ColorMode = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(back, mode, "{mode:?}");
    }
}

#[test]
fn theme_colors_json_explicit_object() {
    let t = ThemeColors {
        blue: 1,
        green: 2,
        purple: 3,
        light_purple: 4,
        royal: 5,
        dark_purple: 6,
    };
    let v: serde_json::Value = serde_json::to_value(&t).unwrap();
    assert_eq!(v["blue"], 1);
    assert_eq!(v["dark_purple"], 6);
}

#[test]
fn sort_mode_json_string_content() {
    let s = serde_json::to_string(&SortMode::Pct).unwrap();
    assert!(s.to_lowercase().contains("pct") || s.contains("Pct"));
}
