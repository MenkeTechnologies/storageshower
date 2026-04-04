//! `palette` for `Matrix` and `Green` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn matrix_six_indexed() {
    let p = palette(ColorMode::Matrix);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn matrix_first_differs_from_green_first() {
    let m = palette(ColorMode::Matrix);
    let g = palette(ColorMode::Green);
    assert_ne!(m.0, g.0);
}
