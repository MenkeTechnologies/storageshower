//! `palette` for `GlitchPop` and `HoloShift` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn glitch_pop_holo_shift_both_six_indexed() {
    for mode in [ColorMode::GlitchPop, ColorMode::HoloShift] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn glitch_pop_first_differs_from_holo_shift_first() {
    let g = palette(ColorMode::GlitchPop);
    let h = palette(ColorMode::HoloShift);
    assert_ne!(g.0, h.0);
}
