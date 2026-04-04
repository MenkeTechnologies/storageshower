//! `palette` for `Green` and `NeonNoir` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn green_neon_noir_both_six_indexed() {
    for mode in [ColorMode::Green, ColorMode::NeonNoir] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn green_first_differs_from_neon_noir_first() {
    let g = palette(ColorMode::Green);
    let n = palette(ColorMode::NeonNoir);
    assert_ne!(g.0, n.0);
}
