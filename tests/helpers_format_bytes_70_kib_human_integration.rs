//! `format_bytes` at 70–71 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn seventy_kib_exactly() {
    assert_eq!(format_bytes(70 * 1024, UnitMode::Human), "70.0K");
}

#[test]
fn seventy_one_kib_exactly() {
    assert_eq!(format_bytes(71 * 1024, UnitMode::Human), "71.0K");
}
