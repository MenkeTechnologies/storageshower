//! `format_rate` for sub–1 byte/s inputs (helpers API).

use storageshower::helpers::format_rate;

#[test]
fn half_byte_per_sec_rounds_to_zero_b_s() {
    assert_eq!(format_rate(0.5), "0B/s");
}

#[test]
fn point_nine_nine_bytes() {
    assert_eq!(format_rate(0.99), "0B/s");
}
