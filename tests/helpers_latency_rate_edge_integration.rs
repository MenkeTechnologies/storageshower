//! `format_latency` / `format_rate` edge cases (public `helpers` API).

use storageshower::helpers::{format_latency, format_rate};

#[test]
fn format_latency_just_below_one_ms() {
    assert_eq!(format_latency(0.99), "<1ms");
}

#[test]
fn format_latency_999_ms_before_second_branch() {
    assert_eq!(format_latency(999.0), "999ms");
}

#[test]
fn format_latency_1000_ms_exactly_one_s() {
    assert_eq!(format_latency(1000.0), "1.0s");
}

#[test]
fn format_latency_large_seconds() {
    let s = format_latency(60_000.0);
    assert!(s.ends_with('s'), "got {s}");
}

#[test]
fn format_rate_exactly_1023_b_per_s() {
    assert_eq!(format_rate(1023.0), "1023B/s");
}

#[test]
fn format_rate_1024_exactly_one_k() {
    assert_eq!(format_rate(1024.0), "1.0K/s");
}

#[test]
fn format_rate_just_below_one_mib_per_s() {
    let s = format_rate(1_048_575.0);
    assert!(s.ends_with("K/s"), "got {s}");
}

#[test]
fn format_rate_just_below_one_gib_per_s() {
    let s = format_rate(1_073_741_823.0);
    assert!(s.ends_with("M/s"), "got {s}");
}

#[test]
fn format_rate_ten_gib_per_s() {
    let s = format_rate(10.0 * 1_073_741_824.0);
    assert_eq!(s, "10.0G/s");
}

#[test]
fn format_rate_negative_is_zero_band() {
    assert_eq!(format_rate(-100.0), "0B/s");
}
