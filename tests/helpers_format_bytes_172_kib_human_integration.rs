//! `format_bytes` at 172–173 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_seventy_two_kib_exactly() {
    assert_eq!(format_bytes(172 * 1024, UnitMode::Human), "172.0K");
}

#[test]
fn one_hundred_seventy_three_kib_exactly() {
    assert_eq!(format_bytes(173 * 1024, UnitMode::Human), "173.0K");
}
