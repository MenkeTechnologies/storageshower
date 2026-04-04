//! `format_bytes` at 18–19 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn eighteen_kib_exactly() {
    assert_eq!(format_bytes(18 * 1024, UnitMode::Human), "18.0K");
}

#[test]
fn nineteen_kib_exactly() {
    assert_eq!(format_bytes(19 * 1024, UnitMode::Human), "19.0K");
}
