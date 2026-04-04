//! `palette` for `BladeRunner` and `VoidWalker` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn blade_runner_six_indexed() {
    let p = palette(ColorMode::BladeRunner);
    for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
        assert!(matches!(c, Color::Indexed(_)));
    }
}

#[test]
fn blade_runner_first_differs_from_void_walker_first() {
    let b = palette(ColorMode::BladeRunner);
    let v = palette(ColorMode::VoidWalker);
    assert_ne!(b.0, v.0);
}
