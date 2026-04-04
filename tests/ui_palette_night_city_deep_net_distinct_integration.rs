//! `palette` for `NightCity` and `DeepNet` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn night_city_deep_net_both_six_indexed() {
    for mode in [ColorMode::NightCity, ColorMode::DeepNet] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn night_city_first_differs_from_deep_net_first() {
    let n = palette(ColorMode::NightCity);
    let d = palette(ColorMode::DeepNet);
    assert_ne!(n.0, d.0);
}
