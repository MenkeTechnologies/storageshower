//! `format_bytes` at 60–61 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn sixty_kib_exactly() {
    assert_eq!(format_bytes(60 * 1024, UnitMode::Human), "60.0K");
}

#[test]
fn sixty_one_kib_exactly() {
    assert_eq!(format_bytes(61 * 1024, UnitMode::Human), "61.0K");
}
