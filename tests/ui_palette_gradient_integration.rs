//! `ui::palette`, `gradient_color_at`, `palette_for_prefs` (crate boundary).

use std::collections::HashMap;

use ratatui::style::Color;

use storageshower::prefs::Prefs;
use storageshower::types::{ColorMode, ThemeColors};
use storageshower::ui::{gradient_color_at, palette, palette_for_prefs};

#[test]
fn palette_default_first_slot_is_indexed() {
    let (a, _, _, _, _, _) = palette(ColorMode::Default);
    assert_eq!(a, Color::Indexed(27));
}

#[test]
fn gradient_frac_low_uses_second_palette_slot() {
    let (_, g, _, _, _, _) = palette(ColorMode::Green);
    assert_eq!(gradient_color_at(0.1, ColorMode::Green), g);
}

#[test]
fn gradient_frac_mid_uses_first_slot_named_blue() {
    let (b, _, _, _, _, _) = palette(ColorMode::Blue);
    assert_eq!(gradient_color_at(0.4, ColorMode::Blue), b);
}

#[test]
fn gradient_frac_high_uses_purple_slot() {
    let (_, _, p, _, _, _) = palette(ColorMode::Purple);
    assert_eq!(gradient_color_at(0.65, ColorMode::Purple), p);
}

#[test]
fn gradient_frac_top_uses_dark_purple() {
    let (_, _, _, _, _, dp) = palette(ColorMode::Cyan);
    assert_eq!(gradient_color_at(0.99, ColorMode::Cyan), dp);
}

#[test]
fn palette_for_prefs_falls_back_to_color_mode() {
    let p = Prefs {
        color_mode: ColorMode::Matrix,
        active_theme: None,
        ..Default::default()
    };
    // Same code path as palette(color_mode) when no active theme
    assert_eq!(palette_for_prefs(&p), palette(ColorMode::Matrix));
}

#[test]
fn palette_for_prefs_custom_theme_orders_colors() {
    let p = Prefs {
        active_theme: Some("mine".into()),
        custom_themes: HashMap::from([(
            "mine".into(),
            ThemeColors {
                blue: 10,
                green: 20,
                purple: 30,
                light_purple: 40,
                royal: 50,
                dark_purple: 60,
            },
        )]),
        ..Default::default()
    };
    let (a, b, c, d, e, f) = palette_for_prefs(&p);
    assert_eq!(a, Color::Indexed(10));
    assert_eq!(b, Color::Indexed(20));
    assert_eq!(c, Color::Indexed(30));
    assert_eq!(d, Color::Indexed(40));
    assert_eq!(e, Color::Indexed(50));
    assert_eq!(f, Color::Indexed(60));
}

#[test]
fn palette_each_neon_branch_is_six_indexed() {
    for mode in [
        ColorMode::LaserGrid,
        ColorMode::GlitchPop,
        ColorMode::NeonNoir,
    ] {
        let t = palette(mode);
        assert!(matches!(t.0, Color::Indexed(_)));
        assert!(matches!(t.5, Color::Indexed(_)));
    }
}
