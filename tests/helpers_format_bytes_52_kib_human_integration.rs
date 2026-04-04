//! `format_bytes` at 52–53 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn fifty_two_kib_exactly() {
    assert_eq!(format_bytes(52 * 1024, UnitMode::Human), "52.0K");
}

#[test]
fn fifty_three_kib_exactly() {
    assert_eq!(format_bytes(53 * 1024, UnitMode::Human), "53.0K");
}
