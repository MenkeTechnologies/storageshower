//! Human `format_bytes` just below and at the Mebibyte boundary.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_below_mebibyte_stays_kilo_band() {
    assert_eq!(format_bytes(1_048_575, UnitMode::Human), "1024.0K");
}

#[test]
fn exactly_one_mebibyte_human() {
    assert_eq!(format_bytes(1_048_576, UnitMode::Human), "1.0M");
}
