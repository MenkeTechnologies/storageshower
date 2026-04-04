//! `palette` for `DarkSignal` and `GlitchPop` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn dark_signal_glitch_pop_both_six_indexed() {
    for mode in [ColorMode::DarkSignal, ColorMode::GlitchPop] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn dark_signal_first_differs_from_glitch_pop_first() {
    let d = palette(ColorMode::DarkSignal);
    let g = palette(ColorMode::GlitchPop);
    assert_ne!(d.0, g.0);
}
