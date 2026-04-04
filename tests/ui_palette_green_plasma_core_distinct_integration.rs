//! `palette` for `Green` and `PlasmaCore` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn green_plasma_core_both_six_indexed() {
    for mode in [ColorMode::Green, ColorMode::PlasmaCore] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn green_first_differs_from_plasma_core_first() {
    let g = palette(ColorMode::Green);
    let p = palette(ColorMode::PlasmaCore);
    assert_ne!(g.0, p.0);
}
