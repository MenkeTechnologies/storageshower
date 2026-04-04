//! `format_bytes` at 22–23 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn twenty_two_kib_exactly() {
    assert_eq!(format_bytes(22 * 1024, UnitMode::Human), "22.0K");
}

#[test]
fn twenty_three_kib_exactly() {
    assert_eq!(format_bytes(23 * 1024, UnitMode::Human), "23.0K");
}
