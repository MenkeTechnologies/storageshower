//! `format_bytes` at 5 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn five_kib_exactly() {
    assert_eq!(format_bytes(5 * 1024, UnitMode::Human), "5.0K");
}

#[test]
fn four_kib_exactly() {
    assert_eq!(format_bytes(4 * 1024, UnitMode::Human), "4.0K");
}
