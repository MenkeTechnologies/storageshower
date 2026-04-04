//! `palette` for `LaserGrid` and `Sunset` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn laser_grid_sunset_both_six_indexed() {
    for mode in [ColorMode::LaserGrid, ColorMode::Sunset] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn laser_grid_first_differs_from_sunset_first() {
    let l = palette(ColorMode::LaserGrid);
    let s = palette(ColorMode::Sunset);
    assert_ne!(l.0, s.0);
}
