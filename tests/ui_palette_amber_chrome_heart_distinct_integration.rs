//! `palette` for `Amber` and `ChromeHeart` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn amber_chrome_heart_both_six_indexed() {
    for mode in [ColorMode::Amber, ColorMode::ChromeHeart] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn amber_first_differs_from_chrome_heart_first() {
    let a = palette(ColorMode::Amber);
    let c = palette(ColorMode::ChromeHeart);
    assert_ne!(a.0, c.0);
}
