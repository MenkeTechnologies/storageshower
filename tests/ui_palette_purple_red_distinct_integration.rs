//! `palette` for `Purple` and `Red` (indexed, first slots differ).

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
fn purple_first_differs_from_red_first() {
    let p = palette(ColorMode::Purple);
    let r = palette(ColorMode::Red);
    assert_ne!(p.0, r.0);
}
