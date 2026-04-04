//! `format_bytes` at 140–141 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_forty_kib_exactly() {
    assert_eq!(format_bytes(140 * 1024, UnitMode::Human), "140.0K");
}

#[test]
fn one_hundred_forty_one_kib_exactly() {
    assert_eq!(format_bytes(141 * 1024, UnitMode::Human), "141.0K");
}
