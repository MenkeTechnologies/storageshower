//! `serde_json` roundtrip for every `ColorMode` variant (crate boundary).

use storageshower::types::ColorMode;

#[test]
fn all_color_modes_json_roundtrip() {
    for &mode in ColorMode::ALL {
        let s = serde_json::to_string(&mode).unwrap();
        let back: ColorMode = serde_json::from_str(&s).unwrap();
        assert_eq!(back, mode, "json={s:?}");
    }
}

#[test]
fn color_mode_json_strings_are_non_empty() {
    let s = serde_json::to_string(&ColorMode::NeonNoir).unwrap();
    assert!(s.len() > 2, "{s}");
}
