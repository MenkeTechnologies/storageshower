//! `format_bytes` at 162–163 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_sixty_two_kib_exactly() {
    assert_eq!(format_bytes(162 * 1024, UnitMode::Human), "162.0K");
}

#[test]
fn one_hundred_sixty_three_kib_exactly() {
    assert_eq!(format_bytes(163 * 1024, UnitMode::Human), "163.0K");
}
