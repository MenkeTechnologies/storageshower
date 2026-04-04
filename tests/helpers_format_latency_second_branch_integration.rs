//! `format_latency` in the seconds branch (helpers API).

use storageshower::helpers::format_latency;

#[test]
fn fifteen_hundred_ms_is_one_point_five_s() {
    assert_eq!(format_latency(1500.0), "1.5s");
}

#[test]
fn three_thousand_ms_is_three_s() {
    assert_eq!(format_latency(3000.0), "3.0s");
}
