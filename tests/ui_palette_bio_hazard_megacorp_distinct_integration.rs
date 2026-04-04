//! `palette` for `BioHazard` and `Megacorp` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn bio_hazard_megacorp_both_six_indexed() {
    for mode in [ColorMode::BioHazard, ColorMode::Megacorp] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn bio_hazard_first_differs_from_megacorp_first() {
    let b = palette(ColorMode::BioHazard);
    let m = palette(ColorMode::Megacorp);
    assert_ne!(b.0, m.0);
}
