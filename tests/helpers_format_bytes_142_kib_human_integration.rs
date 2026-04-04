//! `format_bytes` at 142–143 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_forty_two_kib_exactly() {
    assert_eq!(format_bytes(142 * 1024, UnitMode::Human), "142.0K");
}

#[test]
fn one_hundred_forty_three_kib_exactly() {
    assert_eq!(format_bytes(143 * 1024, UnitMode::Human), "143.0K");
}
