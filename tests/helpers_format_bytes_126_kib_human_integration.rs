//! `format_bytes` at 126–127 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_twenty_six_kib_exactly() {
    assert_eq!(format_bytes(126 * 1024, UnitMode::Human), "126.0K");
}

#[test]
fn one_hundred_twenty_seven_kib_exactly() {
    assert_eq!(format_bytes(127 * 1024, UnitMode::Human), "127.0K");
}
