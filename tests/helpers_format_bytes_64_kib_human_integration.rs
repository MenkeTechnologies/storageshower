//! `format_bytes` at 64–65 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn sixty_four_kib_exactly() {
    assert_eq!(format_bytes(64 * 1024, UnitMode::Human), "64.0K");
}

#[test]
fn sixty_five_kib_exactly() {
    assert_eq!(format_bytes(65 * 1024, UnitMode::Human), "65.0K");
}
