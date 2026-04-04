//! `format_bytes` at 58–59 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn fifty_eight_kib_exactly() {
    assert_eq!(format_bytes(58 * 1024, UnitMode::Human), "58.0K");
}

#[test]
fn fifty_nine_kib_exactly() {
    assert_eq!(format_bytes(59 * 1024, UnitMode::Human), "59.0K");
}
