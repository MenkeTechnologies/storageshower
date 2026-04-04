//! `format_bytes` at 80–81 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn eighty_kib_exactly() {
    assert_eq!(format_bytes(80 * 1024, UnitMode::Human), "80.0K");
}

#[test]
fn eighty_one_kib_exactly() {
    assert_eq!(format_bytes(81 * 1024, UnitMode::Human), "81.0K");
}
