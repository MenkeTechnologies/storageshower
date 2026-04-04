//! Human `format_bytes` just below and at the K boundary.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_below_kilo_stays_bytes() {
    assert_eq!(format_bytes(1023, UnitMode::Human), "1023B");
}

#[test]
fn exactly_kilo_is_one_k() {
    assert_eq!(format_bytes(1024, UnitMode::Human), "1.0K");
}
