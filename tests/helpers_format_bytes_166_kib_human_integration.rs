//! `format_bytes` at 166–167 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_sixty_six_kib_exactly() {
    assert_eq!(format_bytes(166 * 1024, UnitMode::Human), "166.0K");
}

#[test]
fn one_hundred_sixty_seven_kib_exactly() {
    assert_eq!(format_bytes(167 * 1024, UnitMode::Human), "167.0K");
}
