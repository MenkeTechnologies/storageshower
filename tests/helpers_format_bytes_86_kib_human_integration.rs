//! `format_bytes` at 86–87 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn eighty_six_kib_exactly() {
    assert_eq!(format_bytes(86 * 1024, UnitMode::Human), "86.0K");
}

#[test]
fn eighty_seven_kib_exactly() {
    assert_eq!(format_bytes(87 * 1024, UnitMode::Human), "87.0K");
}
