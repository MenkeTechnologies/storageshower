//! `palette` for `BladeRunner` and `ToxicWaste` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn blade_runner_toxic_waste_both_six_indexed() {
    for mode in [ColorMode::BladeRunner, ColorMode::ToxicWaste] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn blade_runner_first_differs_from_toxic_waste_first() {
    let b = palette(ColorMode::BladeRunner);
    let t = palette(ColorMode::ToxicWaste);
    assert_ne!(b.0, t.0);
}
