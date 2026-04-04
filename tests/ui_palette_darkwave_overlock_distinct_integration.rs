//! `palette` for `Darkwave` and `Overlock` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn darkwave_six_indexed() {
    let p = palette(ColorMode::Darkwave);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn darkwave_first_differs_from_overlock_first() {
    let d = palette(ColorMode::Darkwave);
    let o = palette(ColorMode::Overlock);
    assert_ne!(d.0, o.0);
}
