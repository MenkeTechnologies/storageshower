//! Human `format_bytes` around the tera boundary (public helpers API).

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn exactly_one_tebibyte_human() {
    assert_eq!(format_bytes(1_099_511_627_776, UnitMode::Human), "1.0T");
}

#[test]
fn one_below_tebibyte_still_giga_band() {
    assert_eq!(format_bytes(1_099_511_627_775, UnitMode::Human), "1024.0G");
}

#[test]
fn two_tebibytes_human() {
    assert_eq!(format_bytes(2 * 1_099_511_627_776, UnitMode::Human), "2.0T");
}
