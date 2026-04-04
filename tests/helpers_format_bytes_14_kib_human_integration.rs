//! `format_bytes` at 14–15 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn fourteen_kib_exactly() {
    assert_eq!(format_bytes(14 * 1024, UnitMode::Human), "14.0K");
}

#[test]
fn fifteen_kib_exactly() {
    assert_eq!(format_bytes(15 * 1024, UnitMode::Human), "15.0K");
}
