//! `format_bytes` at 74–75 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn seventy_four_kib_exactly() {
    assert_eq!(format_bytes(74 * 1024, UnitMode::Human), "74.0K");
}

#[test]
fn seventy_five_kib_exactly() {
    assert_eq!(format_bytes(75 * 1024, UnitMode::Human), "75.0K");
}
