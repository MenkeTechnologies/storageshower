//! `palette` for `Default` and `Sunset` (first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn default_and_sunset_six_indexed() {
    for mode in [ColorMode::Default, ColorMode::Sunset] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn default_first_differs_from_sunset_first() {
    let d = palette(ColorMode::Default);
    let s = palette(ColorMode::Sunset);
    assert_ne!(d.0, s.0);
}
