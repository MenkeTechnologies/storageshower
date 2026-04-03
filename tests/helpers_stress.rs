//! Integration-level invariants and sweeps for `storageshower::helpers` (complements `src/helpers.rs` unit tests).

use storageshower::helpers::{
    format_bytes, format_latency, format_rate, format_uptime, truncate_mount,
};
use storageshower::types::UnitMode;

#[test]
fn format_bytes_powers_of_two_no_panic_all_modes() {
    for p in 0u32..64 {
        let b = 1u64 << p;
        for mode in [
            UnitMode::Human,
            UnitMode::Bytes,
            UnitMode::MiB,
            UnitMode::GiB,
        ] {
            let s = format_bytes(b, mode);
            assert!(!s.is_empty(), "p={p} mode={mode:?}");
        }
    }
}

#[test]
fn format_bytes_dense_sample_no_panic() {
    for b in (0u64..=10_000).step_by(97) {
        let _ = format_bytes(b, UnitMode::Human);
    }
    for b in (0u64..=20).map(|i| i * 1_048_576 + 13) {
        let _ = format_bytes(b, UnitMode::Human);
    }
}

#[test]
fn format_bytes_human_includes_unit_suffix_or_raw() {
    let s = format_bytes(5, UnitMode::Human);
    assert!(s.ends_with('B'));
    let s = format_bytes(10_000_000_000, UnitMode::Human);
    assert!(
        s.ends_with('T')
            || s.ends_with('G')
            || s.ends_with('M')
            || s.ends_with('K')
            || s.ends_with('B'),
        "unexpected human: {s}"
    );
}

#[test]
fn format_bytes_bytes_mode_always_suffix_b() {
    for b in [0u64, 1, 1023, 1024, u64::MAX] {
        let s = format_bytes(b, UnitMode::Bytes);
        assert!(s.ends_with('B'), "{s}");
    }
}

#[test]
fn format_bytes_gib_mib_have_decimal() {
    assert!(format_bytes(1_073_741_824, UnitMode::GiB).contains('G'));
    assert!(format_bytes(1_048_576, UnitMode::MiB).contains('M'));
}

#[test]
fn format_uptime_zero_and_one_second() {
    assert_eq!(format_uptime(0), "0m");
    assert_eq!(format_uptime(1), "0m");
}

#[test]
fn format_uptime_one_minute_exact() {
    assert_eq!(format_uptime(60), "1m");
}

#[test]
fn format_uptime_one_hour_exact() {
    assert_eq!(format_uptime(3600), "1h0m");
}

#[test]
fn format_uptime_one_day_exact() {
    assert_eq!(format_uptime(86_400), "1d0h0m");
}

#[test]
fn format_uptime_large_nonempty() {
    let s = format_uptime(999_999_999);
    assert!(s.contains('d') || s.contains('h') || s.contains('m'));
}

#[test]
fn format_uptime_sweep_no_empty_string_odd_seconds() {
    for secs in (0..50_000).step_by(3333) {
        let s = format_uptime(secs);
        assert!(!s.is_empty());
        assert!(
            s.ends_with('m') || s.contains('h') || s.contains('d'),
            "bad uptime {s} for {secs}"
        );
    }
}

#[test]
fn format_latency_negative_treated_as_sub_ms() {
    assert_eq!(format_latency(-1.0), "<1ms");
}

#[test]
fn format_latency_just_below_one_ms() {
    assert_eq!(format_latency(0.999), "<1ms");
}

#[test]
fn format_latency_thousands_of_ms() {
    let s = format_latency(500_000.0);
    assert!(s.ends_with('s'));
}

#[test]
fn format_latency_sweep_ms_band() {
    for ms in [1.0, 2.0, 10.0, 99.0, 500.0, 999.0] {
        let s = format_latency(ms);
        assert!(s.ends_with("ms") || s == "<1ms", "{s} for {ms}");
    }
}

#[test]
fn format_rate_negative_is_zero_band() {
    assert_eq!(format_rate(-1.0), "0B/s");
}

#[test]
fn format_rate_tiny_positive_is_zero_band() {
    assert_eq!(format_rate(0.3), "0B/s");
}

#[test]
fn format_rate_one_byte_per_sec() {
    assert_eq!(format_rate(1.0), "1B/s");
}

#[test]
fn format_rate_gigabyte_per_sec() {
    let s = format_rate(2.5 * 1_073_741_824.0);
    assert!(s.contains('G'));
}

#[test]
fn format_rate_sweep_no_panic() {
    for exp in 0i32..40 {
        let x = 2f64.powi(exp);
        let _ = format_rate(x);
        let _ = format_rate(-x);
    }
}

#[test]
fn truncate_mount_ascii_short() {
    assert_eq!(truncate_mount("/a", 10).trim_end(), "/a");
}

#[test]
fn truncate_mount_padding_when_short() {
    let s = truncate_mount("ab", 6);
    assert_eq!(s.len(), 6);
    assert!(s.starts_with("ab"));
}

#[test]
fn truncate_mount_ellipsis_when_long() {
    let s = truncate_mount("/very/long/path/here", 8);
    assert_eq!(s.chars().count(), 8);
    assert!(s.ends_with('\u{2026}'));
}

#[test]
fn truncate_mount_width_zero() {
    assert_eq!(truncate_mount("", 0).chars().count(), 0);
    assert_eq!(truncate_mount("x", 0), "\u{2026}");
}

#[test]
fn truncate_mount_unicode_codepoints() {
    let s = truncate_mount("/🦀/data", 5);
    assert!(s.chars().count() <= 5);
}

#[test]
fn truncate_mount_width_matches_char_count_when_fits() {
    let path = "/usr/local";
    let w = path.chars().count();
    assert_eq!(truncate_mount(path, w), path);
}

#[test]
fn format_bytes_and_rate_consistent_zero() {
    assert_eq!(format_bytes(0, UnitMode::Human), "0B");
    assert_eq!(format_rate(0.0), "0B/s");
}
