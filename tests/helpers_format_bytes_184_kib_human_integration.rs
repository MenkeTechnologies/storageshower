//! `format_bytes` at 184–185 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_eighty_four_kib_exactly() {
    assert_eq!(format_bytes(184 * 1024, UnitMode::Human), "184.0K");
}

#[test]
fn one_hundred_eighty_five_kib_exactly() {
    assert_eq!(format_bytes(185 * 1024, UnitMode::Human), "185.0K");
}
