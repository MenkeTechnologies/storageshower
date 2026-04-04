//! `palette` for `Green` and `Red` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn green_red_both_six_indexed() {
    for mode in [ColorMode::Green, ColorMode::Red] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn green_first_differs_from_red_first() {
    let g = palette(ColorMode::Green);
    let r = palette(ColorMode::Red);
    assert_ne!(g.0, r.0);
}
