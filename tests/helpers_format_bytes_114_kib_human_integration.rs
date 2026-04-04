//! `format_bytes` at 114–115 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_fourteen_kib_exactly() {
    assert_eq!(format_bytes(114 * 1024, UnitMode::Human), "114.0K");
}

#[test]
fn one_hundred_fifteen_kib_exactly() {
    assert_eq!(format_bytes(115 * 1024, UnitMode::Human), "115.0K");
}
