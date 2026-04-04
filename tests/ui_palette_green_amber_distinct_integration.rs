//! `palette` for `Green` and `Amber` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn green_amber_both_six_indexed() {
    for mode in [ColorMode::Green, ColorMode::Amber] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn green_first_differs_from_amber_first() {
    let g = palette(ColorMode::Green);
    let a = palette(ColorMode::Amber);
    assert_ne!(g.0, a.0);
}
