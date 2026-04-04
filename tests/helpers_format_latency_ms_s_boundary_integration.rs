//! `format_latency` at the 999ms vs 1000ms boundary.

use storageshower::helpers::format_latency;

#[test]
fn nine_ninety_nine_ms_stays_ms() {
    assert_eq!(format_latency(999.0), "999ms");
}

#[test]
fn one_thousand_ms_is_one_second() {
    assert_eq!(format_latency(1000.0), "1.0s");
}
