//! `gradient_color_at` for negative and very large `frac` values.

use storageshower::types::ColorMode;
use storageshower::ui::{gradient_color_at, palette};

#[test]
fn negative_frac_uses_low_branch() {
    let (_, g, _, _, _, _) = palette(ColorMode::Sunset);
    assert_eq!(gradient_color_at(-10.0, ColorMode::Sunset), g);
}

#[test]
fn very_large_frac_uses_dark_purple() {
    let (_, _, _, _, _, dp) = palette(ColorMode::Matrix);
    assert_eq!(gradient_color_at(999.0, ColorMode::Matrix), dp);
}
