//! `format_bytes` at 46–47 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn forty_six_kib_exactly() {
    assert_eq!(format_bytes(46 * 1024, UnitMode::Human), "46.0K");
}

#[test]
fn forty_seven_kib_exactly() {
    assert_eq!(format_bytes(47 * 1024, UnitMode::Human), "47.0K");
}
