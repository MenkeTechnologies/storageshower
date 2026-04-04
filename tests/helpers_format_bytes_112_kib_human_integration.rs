//! `format_bytes` at 112–113 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_twelve_kib_exactly() {
    assert_eq!(format_bytes(112 * 1024, UnitMode::Human), "112.0K");
}

#[test]
fn one_hundred_thirteen_kib_exactly() {
    assert_eq!(format_bytes(113 * 1024, UnitMode::Human), "113.0K");
}
