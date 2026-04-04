//! `format_bytes` at 134–135 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_thirty_four_kib_exactly() {
    assert_eq!(format_bytes(134 * 1024, UnitMode::Human), "134.0K");
}

#[test]
fn one_hundred_thirty_five_kib_exactly() {
    assert_eq!(format_bytes(135 * 1024, UnitMode::Human), "135.0K");
}
