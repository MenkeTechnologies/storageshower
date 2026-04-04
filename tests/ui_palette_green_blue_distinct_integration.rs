//! `palette` for `Green` and `Blue` (indexed colors, first slot differs).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn green_six_indexed() {
    let p = palette(ColorMode::Green);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn green_first_differs_from_blue_first() {
    let g = palette(ColorMode::Green);
    let b = palette(ColorMode::Blue);
    assert_ne!(g.0, b.0);
}
