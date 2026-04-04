//! `palette` for `SteelNerve` and `Overlock` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn steel_nerve_overlock_both_six_indexed() {
    for mode in [ColorMode::SteelNerve, ColorMode::Overlock] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn steel_nerve_first_differs_from_overlock_first() {
    let s = palette(ColorMode::SteelNerve);
    let o = palette(ColorMode::Overlock);
    assert_ne!(s.0, o.0);
}
