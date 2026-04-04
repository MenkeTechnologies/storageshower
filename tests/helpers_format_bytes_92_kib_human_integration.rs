//! `format_bytes` at 92–93 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn ninety_two_kib_exactly() {
    assert_eq!(format_bytes(92 * 1024, UnitMode::Human), "92.0K");
}

#[test]
fn ninety_three_kib_exactly() {
    assert_eq!(format_bytes(93 * 1024, UnitMode::Human), "93.0K");
}
