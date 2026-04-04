//! `format_bytes` at 28–29 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn twenty_eight_kib_exactly() {
    assert_eq!(format_bytes(28 * 1024, UnitMode::Human), "28.0K");
}

#[test]
fn twenty_nine_kib_exactly() {
    assert_eq!(format_bytes(29 * 1024, UnitMode::Human), "29.0K");
}
