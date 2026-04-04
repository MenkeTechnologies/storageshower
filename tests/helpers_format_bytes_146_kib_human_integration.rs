//! `format_bytes` at 146–147 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_forty_six_kib_exactly() {
    assert_eq!(format_bytes(146 * 1024, UnitMode::Human), "146.0K");
}

#[test]
fn one_hundred_forty_seven_kib_exactly() {
    assert_eq!(format_bytes(147 * 1024, UnitMode::Human), "147.0K");
}
