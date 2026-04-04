//! `palette_for_prefs` when `active_theme` is set but no matching `custom_themes` entry.

use storageshower::prefs::Prefs;
use storageshower::types::ColorMode;
use storageshower::ui::{palette, palette_for_prefs};

#[test]
fn missing_theme_entry_falls_back_to_color_mode() {
    let p = Prefs {
        color_mode: ColorMode::Red,
        active_theme: Some("does_not_exist".into()),
        ..Default::default()
    };
    assert_eq!(palette_for_prefs(&p), palette(ColorMode::Red));
}

#[test]
fn empty_custom_themes_map_with_active_name_falls_back() {
    let p = Prefs {
        color_mode: ColorMode::CyberFrost,
        active_theme: Some("any".into()),
        ..Default::default()
    };
    assert_eq!(palette_for_prefs(&p), palette(ColorMode::CyberFrost));
}
