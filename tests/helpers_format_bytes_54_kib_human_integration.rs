//! `format_bytes` at 54–55 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn fifty_four_kib_exactly() {
    assert_eq!(format_bytes(54 * 1024, UnitMode::Human), "54.0K");
}

#[test]
fn fifty_five_kib_exactly() {
    assert_eq!(format_bytes(55 * 1024, UnitMode::Human), "55.0K");
}
