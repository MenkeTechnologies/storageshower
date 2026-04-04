//! `format_bytes` at 88–89 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn eighty_eight_kib_exactly() {
    assert_eq!(format_bytes(88 * 1024, UnitMode::Human), "88.0K");
}

#[test]
fn eighty_nine_kib_exactly() {
    assert_eq!(format_bytes(89 * 1024, UnitMode::Human), "89.0K");
}
