//! Human `format_bytes` just below and at the Gibibyte boundary.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn one_below_gibibyte_stays_mega_band() {
    assert_eq!(format_bytes(1_073_741_823, UnitMode::Human), "1024.0M");
}

#[test]
fn exactly_one_gibibyte_human() {
    assert_eq!(format_bytes(1_073_741_824, UnitMode::Human), "1.0G");
}
