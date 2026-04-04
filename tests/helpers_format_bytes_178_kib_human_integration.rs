//! `format_bytes` at 178–179 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_seventy_eight_kib_exactly() {
    assert_eq!(format_bytes(178 * 1024, UnitMode::Human), "178.0K");
}

#[test]
fn one_hundred_seventy_nine_kib_exactly() {
    assert_eq!(format_bytes(179 * 1024, UnitMode::Human), "179.0K");
}
