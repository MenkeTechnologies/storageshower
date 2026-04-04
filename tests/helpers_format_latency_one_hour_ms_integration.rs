//! `format_latency` for one hour expressed in milliseconds.

use storageshower::helpers::format_latency;

#[test]
fn three_six_million_ms_is_3600_seconds() {
    assert_eq!(format_latency(3_600_000.0), "3600.0s");
}
