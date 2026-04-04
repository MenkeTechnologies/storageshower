//! `format_bytes` at 148–149 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_forty_eight_kib_exactly() {
    assert_eq!(format_bytes(148 * 1024, UnitMode::Human), "148.0K");
}

#[test]
fn one_hundred_forty_nine_kib_exactly() {
    assert_eq!(format_bytes(149 * 1024, UnitMode::Human), "149.0K");
}
