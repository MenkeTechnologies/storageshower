//! `format_rate` for multi–gibibyte per second values (helpers API).

use storageshower::helpers::format_rate;

#[test]
fn ten_gib_per_sec() {
    assert_eq!(format_rate(10.0 * 1_073_741_824.0), "10.0G/s");
}

#[test]
fn fractional_gib_per_sec() {
    assert_eq!(format_rate(2.5 * 1_073_741_824.0), "2.5G/s");
}
