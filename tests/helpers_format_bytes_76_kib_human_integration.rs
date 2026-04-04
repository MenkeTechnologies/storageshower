//! `format_bytes` at 76–77 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn seventy_six_kib_exactly() {
    assert_eq!(format_bytes(76 * 1024, UnitMode::Human), "76.0K");
}

#[test]
fn seventy_seven_kib_exactly() {
    assert_eq!(format_bytes(77 * 1024, UnitMode::Human), "77.0K");
}
