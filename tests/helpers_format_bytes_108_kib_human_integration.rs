//! `format_bytes` at 108–109 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_eight_kib_exactly() {
    assert_eq!(format_bytes(108 * 1024, UnitMode::Human), "108.0K");
}

#[test]
fn one_hundred_nine_kib_exactly() {
    assert_eq!(format_bytes(109 * 1024, UnitMode::Human), "109.0K");
}
