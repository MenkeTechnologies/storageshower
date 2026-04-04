//! `palette` for `Cyan` and `Red` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn cyan_six_indexed() {
    let p = palette(ColorMode::Cyan);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn cyan_first_differs_from_red_first() {
    let c = palette(ColorMode::Cyan);
    let r = palette(ColorMode::Red);
    assert_ne!(c.0, r.0);
}
