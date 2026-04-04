//! `format_bytes` at 3 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn three_kib_exactly() {
    assert_eq!(format_bytes(3 * 1024, UnitMode::Human), "3.0K");
}

#[test]
fn two_kib_exactly() {
    assert_eq!(format_bytes(2 * 1024, UnitMode::Human), "2.0K");
}
