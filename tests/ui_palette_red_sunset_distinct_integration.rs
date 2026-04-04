//! `palette` for `Red` and `Sunset` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn red_sunset_both_six_indexed() {
    for mode in [ColorMode::Red, ColorMode::Sunset] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn red_first_differs_from_sunset_first() {
    let r = palette(ColorMode::Red);
    let s = palette(ColorMode::Sunset);
    assert_ne!(r.0, s.0);
}
