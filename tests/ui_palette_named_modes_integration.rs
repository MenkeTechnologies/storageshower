//! `palette` returns indexed colors for selected `ColorMode` variants.

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn megacorp_all_indexed() {
    let p = palette(ColorMode::Megacorp);
    assert!(matches!(p.0, Color::Indexed(_)));
    assert!(matches!(p.5, Color::Indexed(_)));
}

#[test]
fn overlock_all_indexed() {
    let p = palette(ColorMode::Overlock);
    assert!(matches!(p.2, Color::Indexed(_)));
    assert!(matches!(p.4, Color::Indexed(_)));
}

#[test]
fn darkwave_distinct_slots() {
    let p = palette(ColorMode::Darkwave);
    assert_ne!(p.0, p.5);
}
