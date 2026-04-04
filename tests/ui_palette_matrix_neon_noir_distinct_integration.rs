//! `palette` for `Matrix` and `NeonNoir` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn matrix_neon_noir_both_six_indexed() {
    for mode in [ColorMode::Matrix, ColorMode::NeonNoir] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn matrix_first_differs_from_neon_noir_first() {
    let m = palette(ColorMode::Matrix);
    let n = palette(ColorMode::NeonNoir);
    assert_ne!(m.0, n.0);
}
