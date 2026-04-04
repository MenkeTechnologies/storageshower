//! `format_bytes` at 124–125 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_twenty_four_kib_exactly() {
    assert_eq!(format_bytes(124 * 1024, UnitMode::Human), "124.0K");
}

#[test]
fn one_hundred_twenty_five_kib_exactly() {
    assert_eq!(format_bytes(125 * 1024, UnitMode::Human), "125.0K");
}
