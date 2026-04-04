//! `format_bytes` at 56–57 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn fifty_six_kib_exactly() {
    assert_eq!(format_bytes(56 * 1024, UnitMode::Human), "56.0K");
}

#[test]
fn fifty_seven_kib_exactly() {
    assert_eq!(format_bytes(57 * 1024, UnitMode::Human), "57.0K");
}
