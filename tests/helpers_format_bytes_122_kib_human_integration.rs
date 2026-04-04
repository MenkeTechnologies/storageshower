//! `format_bytes` at 122–123 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_twenty_two_kib_exactly() {
    assert_eq!(format_bytes(122 * 1024, UnitMode::Human), "122.0K");
}

#[test]
fn one_hundred_twenty_three_kib_exactly() {
    assert_eq!(format_bytes(123 * 1024, UnitMode::Human), "123.0K");
}
