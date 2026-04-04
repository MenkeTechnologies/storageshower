//! `palette` for `Megacorp` and `Zaibatsu` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn megacorp_six_indexed() {
    let p = palette(ColorMode::Megacorp);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn megacorp_first_differs_from_zaibatsu_first() {
    let m = palette(ColorMode::Megacorp);
    let z = palette(ColorMode::Zaibatsu);
    assert_ne!(m.0, z.0);
}
