//! `format_bytes` at 84–85 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn eighty_four_kib_exactly() {
    assert_eq!(format_bytes(84 * 1024, UnitMode::Human), "84.0K");
}

#[test]
fn eighty_five_kib_exactly() {
    assert_eq!(format_bytes(85 * 1024, UnitMode::Human), "85.0K");
}
