//! `format_bytes` at 118–119 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_eighteen_kib_exactly() {
    assert_eq!(format_bytes(118 * 1024, UnitMode::Human), "118.0K");
}

#[test]
fn one_hundred_nineteen_kib_exactly() {
    assert_eq!(format_bytes(119 * 1024, UnitMode::Human), "119.0K");
}
