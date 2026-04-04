//! `format_bytes` at 130–131 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_thirty_kib_exactly() {
    assert_eq!(format_bytes(130 * 1024, UnitMode::Human), "130.0K");
}

#[test]
fn one_hundred_thirty_one_kib_exactly() {
    assert_eq!(format_bytes(131 * 1024, UnitMode::Human), "131.0K");
}
