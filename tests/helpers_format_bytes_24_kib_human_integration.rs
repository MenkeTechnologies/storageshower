//! `format_bytes` at 24–25 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn twenty_four_kib_exactly() {
    assert_eq!(format_bytes(24 * 1024, UnitMode::Human), "24.0K");
}

#[test]
fn twenty_five_kib_exactly() {
    assert_eq!(format_bytes(25 * 1024, UnitMode::Human), "25.0K");
}
