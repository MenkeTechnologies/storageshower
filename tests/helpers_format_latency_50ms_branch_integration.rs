//! `format_latency` in the 1–999 ms branch (helpers API).

use storageshower::helpers::format_latency;

#[test]
fn fifty_ms_round_trips() {
    assert_eq!(format_latency(50.0), "50ms");
}

#[test]
fn ninety_nine_point_four_ms_rounds_to_99ms() {
    assert_eq!(format_latency(99.4), "99ms");
}
