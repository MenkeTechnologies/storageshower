//! `format_bytes` at 174–175 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_seventy_four_kib_exactly() {
    assert_eq!(format_bytes(174 * 1024, UnitMode::Human), "174.0K");
}

#[test]
fn one_hundred_seventy_five_kib_exactly() {
    assert_eq!(format_bytes(175 * 1024, UnitMode::Human), "175.0K");
}
