//! `palette` for `Default` and `Matrix` (first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn default_six_indexed() {
    let p = palette(ColorMode::Default);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn default_first_differs_from_matrix_first() {
    let d = palette(ColorMode::Default);
    let m = palette(ColorMode::Matrix);
    assert_ne!(d.0, m.0);
}
