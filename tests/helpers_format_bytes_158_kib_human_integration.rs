//! `format_bytes` at 158–159 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_fifty_eight_kib_exactly() {
    assert_eq!(format_bytes(158 * 1024, UnitMode::Human), "158.0K");
}

#[test]
fn one_hundred_fifty_nine_kib_exactly() {
    assert_eq!(format_bytes(159 * 1024, UnitMode::Human), "159.0K");
}
