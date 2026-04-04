//! `palette` for `Amber` and `Cyan` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn amber_six_indexed() {
    let p = palette(ColorMode::Amber);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn amber_first_differs_from_cyan_first() {
    let a = palette(ColorMode::Amber);
    let c = palette(ColorMode::Cyan);
    assert_ne!(a.0, c.0);
}
