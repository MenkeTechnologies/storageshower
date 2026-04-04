//! `format_bytes` at 156–157 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_fifty_six_kib_exactly() {
    assert_eq!(format_bytes(156 * 1024, UnitMode::Human), "156.0K");
}

#[test]
fn one_hundred_fifty_seven_kib_exactly() {
    assert_eq!(format_bytes(157 * 1024, UnitMode::Human), "157.0K");
}
