//! `format_bytes` at 104–105 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_four_kib_exactly() {
    assert_eq!(format_bytes(104 * 1024, UnitMode::Human), "104.0K");
}

#[test]
fn one_hundred_five_kib_exactly() {
    assert_eq!(format_bytes(105 * 1024, UnitMode::Human), "105.0K");
}
