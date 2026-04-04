//! `format_bytes` at 94–95 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn ninety_four_kib_exactly() {
    assert_eq!(format_bytes(94 * 1024, UnitMode::Human), "94.0K");
}

#[test]
fn ninety_five_kib_exactly() {
    assert_eq!(format_bytes(95 * 1024, UnitMode::Human), "95.0K");
}
