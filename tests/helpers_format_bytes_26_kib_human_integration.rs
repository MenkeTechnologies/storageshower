//! `format_bytes` at 26–27 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn twenty_six_kib_exactly() {
    assert_eq!(format_bytes(26 * 1024, UnitMode::Human), "26.0K");
}

#[test]
fn twenty_seven_kib_exactly() {
    assert_eq!(format_bytes(27 * 1024, UnitMode::Human), "27.0K");
}
