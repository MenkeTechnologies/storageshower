//! `format_bytes` at 82–83 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn eighty_two_kib_exactly() {
    assert_eq!(format_bytes(82 * 1024, UnitMode::Human), "82.0K");
}

#[test]
fn eighty_three_kib_exactly() {
    assert_eq!(format_bytes(83 * 1024, UnitMode::Human), "83.0K");
}
