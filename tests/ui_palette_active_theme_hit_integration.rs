//! `palette_for_prefs` when `active_theme` matches a `custom_themes` entry.

use std::collections::HashMap;

use ratatui::style::Color;

use storageshower::prefs::Prefs;
use storageshower::types::{ColorMode, ThemeColors};
use storageshower::ui::{palette, palette_for_prefs};

#[test]
fn uses_custom_theme_colors_when_name_matches() {
    let p = Prefs {
        color_mode: ColorMode::Default,
        active_theme: Some("slot".into()),
        custom_themes: HashMap::from([(
            "slot".into(),
            ThemeColors {
                blue: 40,
                green: 41,
                purple: 42,
                light_purple: 43,
                royal: 44,
                dark_purple: 45,
            },
        )]),
        ..Default::default()
    };
    let pal = palette_for_prefs(&p);
    assert_eq!(pal.0, Color::Indexed(40));
    assert_eq!(pal.5, Color::Indexed(45));
}

#[test]
fn wrong_active_name_ignores_custom_map_uses_builtin() {
    let p = Prefs {
        color_mode: ColorMode::Green,
        active_theme: Some("missing".into()),
        custom_themes: HashMap::from([(
            "other".into(),
            ThemeColors {
                blue: 1,
                green: 2,
                purple: 3,
                light_purple: 4,
                royal: 5,
                dark_purple: 6,
            },
        )]),
        ..Default::default()
    };
    assert_eq!(palette_for_prefs(&p), palette(ColorMode::Green));
}
