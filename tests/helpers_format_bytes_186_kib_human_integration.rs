//! `format_bytes` at 186–187 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_eighty_six_kib_exactly() {
    assert_eq!(format_bytes(186 * 1024, UnitMode::Human), "186.0K");
}

#[test]
fn one_hundred_eighty_seven_kib_exactly() {
    assert_eq!(format_bytes(187 * 1024, UnitMode::Human), "187.0K");
}
