//! `format_bytes` at 66–67 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn sixty_six_kib_exactly() {
    assert_eq!(format_bytes(66 * 1024, UnitMode::Human), "66.0K");
}

#[test]
fn sixty_seven_kib_exactly() {
    assert_eq!(format_bytes(67 * 1024, UnitMode::Human), "67.0K");
}
