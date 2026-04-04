//! `palette` for `BioHazard` and `QuantumFlux`.

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn bio_hazard_six_slots_indexed() {
    let p = palette(ColorMode::BioHazard);
    assert!(matches!(p.0, Color::Indexed(_)));
    assert!(matches!(p.3, Color::Indexed(_)));
}

#[test]
fn quantum_flux_middle_slots_differ() {
    let p = palette(ColorMode::QuantumFlux);
    assert_ne!(p.1, p.4);
}
