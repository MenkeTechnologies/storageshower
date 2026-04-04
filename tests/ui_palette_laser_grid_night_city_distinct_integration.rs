//! `palette` for `LaserGrid` and `NightCity` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn laser_grid_night_city_both_six_indexed() {
    for mode in [ColorMode::LaserGrid, ColorMode::NightCity] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn laser_grid_first_differs_from_night_city_first() {
    let l = palette(ColorMode::LaserGrid);
    let n = palette(ColorMode::NightCity);
    assert_ne!(l.0, n.0);
}
