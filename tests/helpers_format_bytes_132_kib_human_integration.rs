//! `format_bytes` at 132–133 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_thirty_two_kib_exactly() {
    assert_eq!(format_bytes(132 * 1024, UnitMode::Human), "132.0K");
}

#[test]
fn one_hundred_thirty_three_kib_exactly() {
    assert_eq!(format_bytes(133 * 1024, UnitMode::Human), "133.0K");
}
