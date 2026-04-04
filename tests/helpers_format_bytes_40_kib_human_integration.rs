//! `format_bytes` at 40–41 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn forty_kib_exactly() {
    assert_eq!(format_bytes(40 * 1024, UnitMode::Human), "40.0K");
}

#[test]
fn forty_one_kib_exactly() {
    assert_eq!(format_bytes(41 * 1024, UnitMode::Human), "41.0K");
}
