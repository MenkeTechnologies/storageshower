//! `format_bytes` at 100–101 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_kib_exactly() {
    assert_eq!(format_bytes(100 * 1024, UnitMode::Human), "100.0K");
}

#[test]
fn one_hundred_one_kib_exactly() {
    assert_eq!(format_bytes(101 * 1024, UnitMode::Human), "101.0K");
}
