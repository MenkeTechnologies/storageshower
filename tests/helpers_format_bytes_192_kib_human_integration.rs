//! `format_bytes` at 192–193 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_ninety_two_kib_exactly() {
    assert_eq!(format_bytes(192 * 1024, UnitMode::Human), "192.0K");
}

#[test]
fn one_hundred_ninety_three_kib_exactly() {
    assert_eq!(format_bytes(193 * 1024, UnitMode::Human), "193.0K");
}
