//! `format_bytes` at 170–171 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_seventy_kib_exactly() {
    assert_eq!(format_bytes(170 * 1024, UnitMode::Human), "170.0K");
}

#[test]
fn one_hundred_seventy_one_kib_exactly() {
    assert_eq!(format_bytes(171 * 1024, UnitMode::Human), "171.0K");
}
