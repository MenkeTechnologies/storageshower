//! `format_bytes` at 48–49 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn forty_eight_kib_exactly() {
    assert_eq!(format_bytes(48 * 1024, UnitMode::Human), "48.0K");
}

#[test]
fn forty_nine_kib_exactly() {
    assert_eq!(format_bytes(49 * 1024, UnitMode::Human), "49.0K");
}
