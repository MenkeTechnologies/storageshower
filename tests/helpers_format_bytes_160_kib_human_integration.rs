//! `format_bytes` at 160–161 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_sixty_kib_exactly() {
    assert_eq!(format_bytes(160 * 1024, UnitMode::Human), "160.0K");
}

#[test]
fn one_hundred_sixty_one_kib_exactly() {
    assert_eq!(format_bytes(161 * 1024, UnitMode::Human), "161.0K");
}
