//! `format_bytes` at 90–91 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn ninety_kib_exactly() {
    assert_eq!(format_bytes(90 * 1024, UnitMode::Human), "90.0K");
}

#[test]
fn ninety_one_kib_exactly() {
    assert_eq!(format_bytes(91 * 1024, UnitMode::Human), "91.0K");
}
