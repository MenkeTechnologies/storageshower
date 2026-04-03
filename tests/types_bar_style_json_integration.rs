//! `serde_json` for `BarStyle` (integration crate boundary).

use storageshower::types::BarStyle;

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
fn bar_style_json_not_empty_string() {
    let s = serde_json::to_string(&BarStyle::Thin).unwrap();
    assert!(s.len() > 2);
}
