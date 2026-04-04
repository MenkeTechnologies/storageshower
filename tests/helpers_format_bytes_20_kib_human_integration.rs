//! `format_bytes` at 20–21 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn twenty_kib_exactly() {
    assert_eq!(format_bytes(20 * 1024, UnitMode::Human), "20.0K");
}

#[test]
fn twenty_one_kib_exactly() {
    assert_eq!(format_bytes(21 * 1024, UnitMode::Human), "21.0K");
}
