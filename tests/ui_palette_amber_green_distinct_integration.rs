//! `palette` for `Amber` and `Green` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn amber_green_both_six_indexed() {
    for mode in [ColorMode::Amber, ColorMode::Green] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn amber_first_differs_from_green_first() {
    let a = palette(ColorMode::Amber);
    let g = palette(ColorMode::Green);
    assert_ne!(a.0, g.0);
}
