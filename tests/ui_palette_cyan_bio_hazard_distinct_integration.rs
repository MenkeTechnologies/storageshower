//! `palette` for `Cyan` and `BioHazard` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn cyan_bio_hazard_both_six_indexed() {
    for mode in [ColorMode::Cyan, ColorMode::BioHazard] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn cyan_first_differs_from_bio_hazard_first() {
    let c = palette(ColorMode::Cyan);
    let b = palette(ColorMode::BioHazard);
    assert_ne!(c.0, b.0);
}
