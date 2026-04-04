//! `format_bytes` at 8–9 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn eight_kib_exactly() {
    assert_eq!(format_bytes(8 * 1024, UnitMode::Human), "8.0K");
}

#[test]
fn nine_kib_exactly() {
    assert_eq!(format_bytes(9 * 1024, UnitMode::Human), "9.0K");
}
