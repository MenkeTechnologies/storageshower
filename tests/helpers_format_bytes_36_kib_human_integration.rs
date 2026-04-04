//! `format_bytes` at 36–37 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn thirty_six_kib_exactly() {
    assert_eq!(format_bytes(36 * 1024, UnitMode::Human), "36.0K");
}

#[test]
fn thirty_seven_kib_exactly() {
    assert_eq!(format_bytes(37 * 1024, UnitMode::Human), "37.0K");
}
