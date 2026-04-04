//! `palette` for `Sakura` and `Sunset` (distinct slots).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn sakura_six_indexed() {
    let p = palette(ColorMode::Sakura);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn sunset_first_differs_from_last() {
    let p = palette(ColorMode::Sunset);
    assert_ne!(p.0, p.5);
}
