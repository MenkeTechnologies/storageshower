//! `palette` for `Sunset` and `VoidWalker` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn sunset_void_walker_both_six_indexed() {
    for mode in [ColorMode::Sunset, ColorMode::VoidWalker] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn sunset_first_differs_from_void_walker_first() {
    let s = palette(ColorMode::Sunset);
    let v = palette(ColorMode::VoidWalker);
    assert_ne!(s.0, v.0);
}
