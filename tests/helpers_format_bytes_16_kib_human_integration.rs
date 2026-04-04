//! `format_bytes` at 16–17 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn sixteen_kib_exactly() {
    assert_eq!(format_bytes(16 * 1024, UnitMode::Human), "16.0K");
}

#[test]
fn seventeen_kib_exactly() {
    assert_eq!(format_bytes(17 * 1024, UnitMode::Human), "17.0K");
}
