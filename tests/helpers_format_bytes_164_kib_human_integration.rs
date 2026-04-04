//! `format_bytes` at 164–165 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_sixty_four_kib_exactly() {
    assert_eq!(format_bytes(164 * 1024, UnitMode::Human), "164.0K");
}

#[test]
fn one_hundred_sixty_five_kib_exactly() {
    assert_eq!(format_bytes(165 * 1024, UnitMode::Human), "165.0K");
}
