//! `format_bytes` at 128–129 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_twenty_eight_kib_exactly() {
    assert_eq!(format_bytes(128 * 1024, UnitMode::Human), "128.0K");
}

#[test]
fn one_hundred_twenty_nine_kib_exactly() {
    assert_eq!(format_bytes(129 * 1024, UnitMode::Human), "129.0K");
}
