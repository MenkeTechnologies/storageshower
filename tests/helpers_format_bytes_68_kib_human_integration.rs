//! `format_bytes` at 68–69 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn sixty_eight_kib_exactly() {
    assert_eq!(format_bytes(68 * 1024, UnitMode::Human), "68.0K");
}

#[test]
fn sixty_nine_kib_exactly() {
    assert_eq!(format_bytes(69 * 1024, UnitMode::Human), "69.0K");
}
