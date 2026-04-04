//! `palette` for `Matrix` and `Megacorp` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn matrix_megacorp_both_six_indexed() {
    for mode in [ColorMode::Matrix, ColorMode::Megacorp] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn matrix_first_differs_from_megacorp_first() {
    let m = palette(ColorMode::Matrix);
    let g = palette(ColorMode::Megacorp);
    assert_ne!(m.0, g.0);
}
