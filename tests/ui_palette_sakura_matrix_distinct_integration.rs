//! `palette` for `Sakura` and `Matrix` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn sakura_matrix_both_six_indexed() {
    let s = palette(ColorMode::Sakura);
    let m = palette(ColorMode::Matrix);
    for p in [s, m] {
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn sakura_first_differs_from_matrix_first() {
    let s = palette(ColorMode::Sakura);
    let m = palette(ColorMode::Matrix);
    assert_ne!(s.0, m.0);
}
