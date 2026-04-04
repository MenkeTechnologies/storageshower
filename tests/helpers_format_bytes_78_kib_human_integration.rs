//! `format_bytes` at 78–79 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn seventy_eight_kib_exactly() {
    assert_eq!(format_bytes(78 * 1024, UnitMode::Human), "78.0K");
}

#[test]
fn seventy_nine_kib_exactly() {
    assert_eq!(format_bytes(79 * 1024, UnitMode::Human), "79.0K");
}
