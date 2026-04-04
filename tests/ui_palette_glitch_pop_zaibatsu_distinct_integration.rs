//! `palette` for `GlitchPop` and `Zaibatsu` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn glitch_pop_zaibatsu_both_six_indexed() {
    for mode in [ColorMode::GlitchPop, ColorMode::Zaibatsu] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn glitch_pop_first_differs_from_zaibatsu_first() {
    let g = palette(ColorMode::GlitchPop);
    let z = palette(ColorMode::Zaibatsu);
    assert_ne!(g.0, z.0);
}
