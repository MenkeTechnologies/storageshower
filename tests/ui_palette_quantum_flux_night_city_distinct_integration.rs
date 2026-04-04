//! `palette` for `QuantumFlux` and `NightCity` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn quantum_flux_night_city_both_six_indexed() {
    for mode in [ColorMode::QuantumFlux, ColorMode::NightCity] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn quantum_flux_first_differs_from_night_city_first() {
    let q = palette(ColorMode::QuantumFlux);
    let n = palette(ColorMode::NightCity);
    assert_ne!(q.0, n.0);
}
