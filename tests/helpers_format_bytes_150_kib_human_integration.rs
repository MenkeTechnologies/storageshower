//! `format_bytes` at 150–151 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_fifty_kib_exactly() {
    assert_eq!(format_bytes(150 * 1024, UnitMode::Human), "150.0K");
}

#[test]
fn one_hundred_fifty_one_kib_exactly() {
    assert_eq!(format_bytes(151 * 1024, UnitMode::Human), "151.0K");
}
