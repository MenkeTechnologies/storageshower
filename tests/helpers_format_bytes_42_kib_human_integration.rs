//! `format_bytes` at 42–43 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn forty_two_kib_exactly() {
    assert_eq!(format_bytes(42 * 1024, UnitMode::Human), "42.0K");
}

#[test]
fn forty_three_kib_exactly() {
    assert_eq!(format_bytes(43 * 1024, UnitMode::Human), "43.0K");
}
