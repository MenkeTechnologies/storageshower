//! `format_bytes` at 98–99 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn ninety_eight_kib_exactly() {
    assert_eq!(format_bytes(98 * 1024, UnitMode::Human), "98.0K");
}

#[test]
fn ninety_nine_kib_exactly() {
    assert_eq!(format_bytes(99 * 1024, UnitMode::Human), "99.0K");
}
