//! `format_bytes` at 180–181 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_eighty_kib_exactly() {
    assert_eq!(format_bytes(180 * 1024, UnitMode::Human), "180.0K");
}

#[test]
fn one_hundred_eighty_one_kib_exactly() {
    assert_eq!(format_bytes(181 * 1024, UnitMode::Human), "181.0K");
}
