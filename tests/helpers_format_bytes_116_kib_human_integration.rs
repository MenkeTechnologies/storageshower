//! `format_bytes` at 116–117 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_sixteen_kib_exactly() {
    assert_eq!(format_bytes(116 * 1024, UnitMode::Human), "116.0K");
}

#[test]
fn one_hundred_seventeen_kib_exactly() {
    assert_eq!(format_bytes(117 * 1024, UnitMode::Human), "117.0K");
}
