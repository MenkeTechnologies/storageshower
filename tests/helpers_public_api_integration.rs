//! Integration coverage for `storageshower::helpers` (public crate API).
//! Complements `src/helpers.rs` unit tests with table-driven cases from an external crate.

use storageshower::helpers::{
    format_bytes, format_latency, format_rate, format_uptime, truncate_mount,
};
use storageshower::types::UnitMode;

#[test]
fn format_bytes_human_matrix_from_crate_root() {
    let cases: &[(u64, &str)] = &[
        (0, "0B"),
        (1, "1B"),
        (1023, "1023B"),
        (1024, "1.0K"),
        (1_048_576, "1.0M"),
        (1_073_741_824, "1.0G"),
        (1_099_511_627_776, "1.0T"),
    ];
    for &(n, want) in cases {
        assert_eq!(format_bytes(n, UnitMode::Human), want, "human {n}");
    }
}

#[test]
fn format_bytes_explicit_modes_from_crate_root() {
    assert_eq!(format_bytes(1_048_576, UnitMode::MiB), "1.0M");
    assert_eq!(format_bytes(1_073_741_824, UnitMode::GiB), "1.0G");
    assert_eq!(format_bytes(42, UnitMode::Bytes), "42B");
}

#[test]
fn format_uptime_branches_from_crate_root() {
    assert_eq!(format_uptime(59), "0m");
    assert_eq!(format_uptime(3600), "1h0m");
    assert_eq!(format_uptime(86400), "1d0h0m");
    assert_eq!(format_uptime(86400 + 3661), "1d1h1m");
}

#[test]
fn format_latency_sub_ms_and_ms_and_seconds() {
    assert_eq!(format_latency(0.0), "<1ms");
    assert_eq!(format_latency(42.0), "42ms");
    assert_eq!(format_latency(1000.0), "1.0s");
    assert_eq!(format_latency(3_600_000.0), "3600.0s");
}

#[test]
fn format_rate_tier_boundaries_from_crate_root() {
    assert_eq!(format_rate(0.0), "0B/s");
    assert_eq!(format_rate(512.0), "512B/s");
    assert_eq!(format_rate(1024.0), "1.0K/s");
    assert_eq!(format_rate(1_048_576.0), "1.0M/s");
    assert_eq!(format_rate(1_073_741_824.0), "1.0G/s");
}

#[test]
fn format_rate_sub_one_byte_per_sec_is_zero_band() {
    assert_eq!(format_rate(0.5), "0B/s");
}

#[test]
fn truncate_mount_padding_and_ellipsis_from_crate_root() {
    assert_eq!(truncate_mount("/a", 4).as_str(), "/a  ");
    let long = truncate_mount("/very/long/mount/point", 12);
    assert_eq!(long.chars().count(), 12);
    assert!(long.ends_with('\u{2026}'));
}

#[test]
fn truncate_mount_unicode_width_respected() {
    let r = truncate_mount("/日本語/パス", 6);
    assert!(r.chars().count() <= 6);
}

#[test]
fn format_bytes_human_just_below_tera() {
    assert_eq!(format_bytes(1_099_511_627_775, UnitMode::Human), "1024.0G");
}

#[test]
fn format_latency_ms_to_s_boundary() {
    assert_eq!(format_latency(999.0), "999ms");
    assert_eq!(format_latency(1000.0), "1.0s");
}

#[test]
fn format_rate_negative_coerces_to_zero_band() {
    assert_eq!(format_rate(-1.0), "0B/s");
}

#[test]
fn format_uptime_only_minutes_branch() {
    assert_eq!(format_uptime(3599), "59m");
}

#[test]
fn format_uptime_two_days() {
    assert_eq!(format_uptime(2 * 86400), "2d0h0m");
}

#[test]
fn format_bytes_gib_half() {
    assert_eq!(format_bytes(536_870_912, UnitMode::GiB), "0.5G");
}

#[test]
fn format_rate_just_under_one_gib_per_sec() {
    let s = format_rate(1_073_741_823.0);
    assert!(s.ends_with("M/s"), "got {s}");
}

#[test]
fn format_latency_fractional_ms_rounds_display() {
    assert_eq!(format_latency(12.6), "13ms");
}

#[test]
fn truncate_mount_width_zero() {
    let r = truncate_mount("/ab", 0);
    assert_eq!(r, "\u{2026}");
}

#[test]
fn format_bytes_max_u64_no_panic() {
    let _ = format_bytes(u64::MAX, UnitMode::Human);
    let _ = format_bytes(u64::MAX, UnitMode::Bytes);
}
