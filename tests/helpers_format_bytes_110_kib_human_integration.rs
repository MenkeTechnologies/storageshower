//! `format_bytes` at 110–111 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_ten_kib_exactly() {
    assert_eq!(format_bytes(110 * 1024, UnitMode::Human), "110.0K");
}

#[test]
fn one_hundred_eleven_kib_exactly() {
    assert_eq!(format_bytes(111 * 1024, UnitMode::Human), "111.0K");
}
