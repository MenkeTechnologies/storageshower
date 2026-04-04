//! `format_rate` at the 1023 vs 1024 byte/s boundary (helpers API).

use storageshower::helpers::format_rate;

#[test]
fn exactly_1024_bytes_per_sec_is_one_k_per_sec() {
    assert_eq!(format_rate(1024.0), "1.0K/s");
}

#[test]
fn one_below_1024_stays_byte_band() {
    assert_eq!(format_rate(1023.0), "1023B/s");
}
