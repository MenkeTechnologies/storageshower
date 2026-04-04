//! `format_bytes` at 62–63 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn sixty_two_kib_exactly() {
    assert_eq!(format_bytes(62 * 1024, UnitMode::Human), "62.0K");
}

#[test]
fn sixty_three_kib_exactly() {
    assert_eq!(format_bytes(63 * 1024, UnitMode::Human), "63.0K");
}
