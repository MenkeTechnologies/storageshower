//! `format_bytes` at 6–7 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn six_kib_exactly() {
    assert_eq!(format_bytes(6 * 1024, UnitMode::Human), "6.0K");
}

#[test]
fn seven_kib_exactly() {
    assert_eq!(format_bytes(7 * 1024, UnitMode::Human), "7.0K");
}
