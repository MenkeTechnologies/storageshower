//! `palette` for `PlasmaCore` and `NightCity` (indexed colors).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn plasma_core_six_indexed() {
    let p = palette(ColorMode::PlasmaCore);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn night_city_first_differs_from_last() {
    let p = palette(ColorMode::NightCity);
    assert_ne!(p.0, p.5);
}
