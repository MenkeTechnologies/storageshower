//! `format_bytes` at 38–39 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn thirty_eight_kib_exactly() {
    assert_eq!(format_bytes(38 * 1024, UnitMode::Human), "38.0K");
}

#[test]
fn thirty_nine_kib_exactly() {
    assert_eq!(format_bytes(39 * 1024, UnitMode::Human), "39.0K");
}
