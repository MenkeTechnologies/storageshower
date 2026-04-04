//! `format_bytes` at 50–51 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn fifty_kib_exactly() {
    assert_eq!(format_bytes(50 * 1024, UnitMode::Human), "50.0K");
}

#[test]
fn fifty_one_kib_exactly() {
    assert_eq!(format_bytes(51 * 1024, UnitMode::Human), "51.0K");
}
