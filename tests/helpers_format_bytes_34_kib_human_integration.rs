//! `format_bytes` at 34–35 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn thirty_four_kib_exactly() {
    assert_eq!(format_bytes(34 * 1024, UnitMode::Human), "34.0K");
}

#[test]
fn thirty_five_kib_exactly() {
    assert_eq!(format_bytes(35 * 1024, UnitMode::Human), "35.0K");
}
