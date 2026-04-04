//! `format_bytes` at 188–189 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_eighty_eight_kib_exactly() {
    assert_eq!(format_bytes(188 * 1024, UnitMode::Human), "188.0K");
}

#[test]
fn one_hundred_eighty_nine_kib_exactly() {
    assert_eq!(format_bytes(189 * 1024, UnitMode::Human), "189.0K");
}
