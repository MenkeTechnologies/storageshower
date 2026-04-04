//! `palette` for `NeonNoir` and `CyberFrost` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn neon_noir_cyber_frost_both_six_indexed() {
    for mode in [ColorMode::NeonNoir, ColorMode::CyberFrost] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn neon_noir_first_differs_from_cyber_frost_first() {
    let n = palette(ColorMode::NeonNoir);
    let c = palette(ColorMode::CyberFrost);
    assert_ne!(n.0, c.0);
}
