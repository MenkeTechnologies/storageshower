//! `format_bytes` at 96–97 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn ninety_six_kib_exactly() {
    assert_eq!(format_bytes(96 * 1024, UnitMode::Human), "96.0K");
}

#[test]
fn ninety_seven_kib_exactly() {
    assert_eq!(format_bytes(97 * 1024, UnitMode::Human), "97.0K");
}
