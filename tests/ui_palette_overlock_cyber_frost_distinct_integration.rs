//! `palette` for `Overlock` and `CyberFrost` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn overlock_cyber_frost_both_six_indexed() {
    for mode in [ColorMode::Overlock, ColorMode::CyberFrost] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn overlock_first_differs_from_cyber_frost_first() {
    let o = palette(ColorMode::Overlock);
    let c = palette(ColorMode::CyberFrost);
    assert_ne!(o.0, c.0);
}
