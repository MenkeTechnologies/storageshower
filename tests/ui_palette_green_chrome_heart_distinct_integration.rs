//! `palette` for `Green` and `ChromeHeart` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn green_chrome_heart_both_six_indexed() {
    for mode in [ColorMode::Green, ColorMode::ChromeHeart] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn green_first_differs_from_chrome_heart_first() {
    let g = palette(ColorMode::Green);
    let c = palette(ColorMode::ChromeHeart);
    assert_ne!(g.0, c.0);
}
