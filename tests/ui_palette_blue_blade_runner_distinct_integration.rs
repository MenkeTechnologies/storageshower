//! `palette` for `Blue` and `BladeRunner` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn blue_blade_runner_both_six_indexed() {
    for mode in [ColorMode::Blue, ColorMode::BladeRunner] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn blue_first_differs_from_blade_runner_first() {
    let b = palette(ColorMode::Blue);
    let r = palette(ColorMode::BladeRunner);
    assert_ne!(b.0, r.0);
}
