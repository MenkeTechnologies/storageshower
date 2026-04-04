//! `format_bytes` at 10–11 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn ten_kib_exactly() {
    assert_eq!(format_bytes(10 * 1024, UnitMode::Human), "10.0K");
}

#[test]
fn eleven_kib_exactly() {
    assert_eq!(format_bytes(11 * 1024, UnitMode::Human), "11.0K");
}
