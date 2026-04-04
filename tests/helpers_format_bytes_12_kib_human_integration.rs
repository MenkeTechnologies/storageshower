//! `format_bytes` at 12–13 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn twelve_kib_exactly() {
    assert_eq!(format_bytes(12 * 1024, UnitMode::Human), "12.0K");
}

#[test]
fn thirteen_kib_exactly() {
    assert_eq!(format_bytes(13 * 1024, UnitMode::Human), "13.0K");
}
