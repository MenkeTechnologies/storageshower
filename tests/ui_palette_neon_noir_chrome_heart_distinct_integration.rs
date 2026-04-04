//! `palette` for `NeonNoir` and `ChromeHeart` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn neon_noir_six_indexed() {
    let p = palette(ColorMode::NeonNoir);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn neon_noir_first_differs_from_chrome_heart_first() {
    let n = palette(ColorMode::NeonNoir);
    let c = palette(ColorMode::ChromeHeart);
    assert_ne!(n.0, c.0);
}
