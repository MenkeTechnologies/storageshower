//! `format_bytes` at 154–155 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_fifty_four_kib_exactly() {
    assert_eq!(format_bytes(154 * 1024, UnitMode::Human), "154.0K");
}

#[test]
fn one_hundred_fifty_five_kib_exactly() {
    assert_eq!(format_bytes(155 * 1024, UnitMode::Human), "155.0K");
}
