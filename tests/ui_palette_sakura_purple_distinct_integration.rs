//! `palette` for `Sakura` and `Purple` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn sakura_purple_both_six_indexed() {
    for mode in [ColorMode::Sakura, ColorMode::Purple] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn sakura_first_differs_from_purple_first() {
    let s = palette(ColorMode::Sakura);
    let p = palette(ColorMode::Purple);
    assert_ne!(s.0, p.0);
}
