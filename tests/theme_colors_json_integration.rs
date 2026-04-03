//! `serde_json` roundtrip for `ThemeColors` (custom themes in prefs).

use storageshower::types::ThemeColors;

#[test]
fn theme_colors_json_roundtrip_all_channels() {
    let t = ThemeColors {
        blue: 0,
        green: 255,
        purple: 128,
        light_purple: 64,
        royal: 32,
        dark_purple: 16,
    };
    let s = serde_json::to_string(&t).expect("serialize");
    let u: ThemeColors = serde_json::from_str(&s).expect("deserialize");
    assert_eq!(u.blue, t.blue);
    assert_eq!(u.green, t.green);
    assert_eq!(u.purple, t.purple);
    assert_eq!(u.light_purple, t.light_purple);
    assert_eq!(u.royal, t.royal);
    assert_eq!(u.dark_purple, t.dark_purple);
}

#[test]
fn theme_colors_json_preserves_field_names() {
    let s = r#"{"blue":10,"green":20,"purple":30,"light_purple":40,"royal":50,"dark_purple":60}"#;
    let t: ThemeColors = serde_json::from_str(s).expect("deserialize");
    assert_eq!(t.blue, 10);
    assert_eq!(t.dark_purple, 60);
}
