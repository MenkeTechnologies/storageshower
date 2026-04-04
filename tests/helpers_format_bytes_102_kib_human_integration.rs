//! `format_bytes` at 102–103 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_two_kib_exactly() {
    assert_eq!(format_bytes(102 * 1024, UnitMode::Human), "102.0K");
}

#[test]
fn one_hundred_three_kib_exactly() {
    assert_eq!(format_bytes(103 * 1024, UnitMode::Human), "103.0K");
}
