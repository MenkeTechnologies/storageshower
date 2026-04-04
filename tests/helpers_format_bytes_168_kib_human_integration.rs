//! `format_bytes` at 168–169 KiB in `UnitMode::Human`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_hundred_sixty_eight_kib_exactly() {
    assert_eq!(format_bytes(168 * 1024, UnitMode::Human), "168.0K");
}

#[test]
fn one_hundred_sixty_nine_kib_exactly() {
    assert_eq!(format_bytes(169 * 1024, UnitMode::Human), "169.0K");
}
