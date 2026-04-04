//! `format_bytes` at 144–145 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_forty_four_kib_exactly() {
    assert_eq!(format_bytes(144 * 1024, UnitMode::Human), "144.0K");
}

#[test]
fn one_hundred_forty_five_kib_exactly() {
    assert_eq!(format_bytes(145 * 1024, UnitMode::Human), "145.0K");
}
