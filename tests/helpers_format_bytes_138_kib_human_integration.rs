//! `format_bytes` at 138–139 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_thirty_eight_kib_exactly() {
    assert_eq!(format_bytes(138 * 1024, UnitMode::Human), "138.0K");
}

#[test]
fn one_hundred_thirty_nine_kib_exactly() {
    assert_eq!(format_bytes(139 * 1024, UnitMode::Human), "139.0K");
}
