//! `palette` for `DeepNet` and `DarkSignal` (indexed, first slots differ).

use ratatui::style::Color;

use storageshower::types::ColorMode;
use storageshower::ui::palette;

#[test]
fn deep_net_dark_signal_both_six_indexed() {
    for mode in [ColorMode::DeepNet, ColorMode::DarkSignal] {
        let p = palette(mode);
        for c in [p.0, p.1, p.2, p.3, p.4, p.5] {
            assert!(matches!(c, Color::Indexed(_)));
        }
    }
}

#[test]
fn deep_net_first_differs_from_dark_signal_first() {
    let d = palette(ColorMode::DeepNet);
    let s = palette(ColorMode::DarkSignal);
    assert_ne!(d.0, s.0);
}
