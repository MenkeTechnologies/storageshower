//! `palette` for `Cyan` and `Blue` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn cyan_blue_both_six_indexed() {
    for mode in [ColorMode::Cyan, ColorMode::Blue] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn cyan_first_differs_from_blue_first() {
    let c = palette(ColorMode::Cyan);
    let b = palette(ColorMode::Blue);
    assert_ne!(c.0, b.0);
}
