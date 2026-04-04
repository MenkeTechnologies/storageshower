//! `format_bytes` at 106–107 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_six_kib_exactly() {
    assert_eq!(format_bytes(106 * 1024, UnitMode::Human), "106.0K");
}

#[test]
fn one_hundred_seven_kib_exactly() {
    assert_eq!(format_bytes(107 * 1024, UnitMode::Human), "107.0K");
}
