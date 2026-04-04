//! `format_bytes` at 44–45 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn forty_four_kib_exactly() {
    assert_eq!(format_bytes(44 * 1024, UnitMode::Human), "44.0K");
}

#[test]
fn forty_five_kib_exactly() {
    assert_eq!(format_bytes(45 * 1024, UnitMode::Human), "45.0K");
}
