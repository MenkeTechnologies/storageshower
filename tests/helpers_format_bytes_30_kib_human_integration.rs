//! `format_bytes` at 30–31 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn thirty_kib_exactly() {
    assert_eq!(format_bytes(30 * 1024, UnitMode::Human), "30.0K");
}

#[test]
fn thirty_one_kib_exactly() {
    assert_eq!(format_bytes(31 * 1024, UnitMode::Human), "31.0K");
}
