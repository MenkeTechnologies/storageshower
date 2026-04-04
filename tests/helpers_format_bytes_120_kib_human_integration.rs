//! `format_bytes` at 120–121 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_twenty_kib_exactly() {
    assert_eq!(format_bytes(120 * 1024, UnitMode::Human), "120.0K");
}

#[test]
fn one_hundred_twenty_one_kib_exactly() {
    assert_eq!(format_bytes(121 * 1024, UnitMode::Human), "121.0K");
}
