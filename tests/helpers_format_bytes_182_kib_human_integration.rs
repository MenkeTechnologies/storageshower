//! `format_bytes` at 182–183 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_eighty_two_kib_exactly() {
    assert_eq!(format_bytes(182 * 1024, UnitMode::Human), "182.0K");
}

#[test]
fn one_hundred_eighty_three_kib_exactly() {
    assert_eq!(format_bytes(183 * 1024, UnitMode::Human), "183.0K");
}
