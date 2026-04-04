//! `palette` for `Blue` and `Purple` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn blue_purple_both_six_indexed() {
    for mode in [ColorMode::Blue, ColorMode::Purple] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn blue_first_differs_from_purple_first() {
    let b = palette(ColorMode::Blue);
    let p = palette(ColorMode::Purple);
    assert_ne!(b.0, p.0);
}
