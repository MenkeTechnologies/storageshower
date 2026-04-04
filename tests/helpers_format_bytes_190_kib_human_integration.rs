//! `format_bytes` at 190–191 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_ninety_kib_exactly() {
    assert_eq!(format_bytes(190 * 1024, UnitMode::Human), "190.0K");
}

#[test]
fn one_hundred_ninety_one_kib_exactly() {
    assert_eq!(format_bytes(191 * 1024, UnitMode::Human), "191.0K");
}
