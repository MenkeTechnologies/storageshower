//! `format_bytes` at 72–73 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn seventy_two_kib_exactly() {
    assert_eq!(format_bytes(72 * 1024, UnitMode::Human), "72.0K");
}

#[test]
fn seventy_three_kib_exactly() {
    assert_eq!(format_bytes(73 * 1024, UnitMode::Human), "73.0K");
}
