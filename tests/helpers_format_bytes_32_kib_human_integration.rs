//! `format_bytes` at 32–33 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn thirty_two_kib_exactly() {
    assert_eq!(format_bytes(32 * 1024, UnitMode::Human), "32.0K");
}

#[test]
fn thirty_three_kib_exactly() {
    assert_eq!(format_bytes(33 * 1024, UnitMode::Human), "33.0K");
}
