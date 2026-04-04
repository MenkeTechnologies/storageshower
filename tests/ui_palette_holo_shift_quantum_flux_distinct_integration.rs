//! `palette` for `HoloShift` and `QuantumFlux` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn holo_shift_quantum_flux_both_six_indexed() {
    for mode in [ColorMode::HoloShift, ColorMode::QuantumFlux] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn holo_shift_first_differs_from_quantum_flux_first() {
    let h = palette(ColorMode::HoloShift);
    let q = palette(ColorMode::QuantumFlux);
    assert_ne!(h.0, q.0);
}
