//! `format_bytes` at 152–153 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_fifty_two_kib_exactly() {
    assert_eq!(format_bytes(152 * 1024, UnitMode::Human), "152.0K");
}

#[test]
fn one_hundred_fifty_three_kib_exactly() {
    assert_eq!(format_bytes(153 * 1024, UnitMode::Human), "153.0K");
}
