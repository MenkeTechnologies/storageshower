//! `palette` for `BladeRunner` and `CyberFrost` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn blade_runner_cyber_frost_both_six_indexed() {
    for mode in [ColorMode::BladeRunner, ColorMode::CyberFrost] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn blade_runner_first_differs_from_cyber_frost_first() {
    let b = palette(ColorMode::BladeRunner);
    let c = palette(ColorMode::CyberFrost);
    assert_ne!(b.0, c.0);
}
