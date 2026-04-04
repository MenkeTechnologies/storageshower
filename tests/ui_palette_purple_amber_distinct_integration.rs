//! `palette` for `Purple` and `Amber` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn purple_six_indexed() {
    let p = palette(ColorMode::Purple);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn purple_first_differs_from_amber_first() {
    let p = palette(ColorMode::Purple);
    let a = palette(ColorMode::Amber);
    assert_ne!(p.0, a.0);
}
