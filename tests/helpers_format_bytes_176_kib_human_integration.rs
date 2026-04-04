//! `format_bytes` at 176–177 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_seventy_six_kib_exactly() {
    assert_eq!(format_bytes(176 * 1024, UnitMode::Human), "176.0K");
}

#[test]
fn one_hundred_seventy_seven_kib_exactly() {
    assert_eq!(format_bytes(177 * 1024, UnitMode::Human), "177.0K");
}
