//! `format_bytes` at 136–137 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_thirty_six_kib_exactly() {
    assert_eq!(format_bytes(136 * 1024, UnitMode::Human), "136.0K");
}

#[test]
fn one_hundred_thirty_seven_kib_exactly() {
    assert_eq!(format_bytes(137 * 1024, UnitMode::Human), "137.0K");
}
