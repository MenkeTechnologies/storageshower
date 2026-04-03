//! Table-driven `helpers` formatting from the public crate API.

use storageshower::helpers::{format_bytes, format_latency, format_rate, format_uptime};
use storageshower::types::UnitMode;

#[test]
fn format_uptime_table() {
    let cases: &[(u64, &str)] = &[
        (0, "0m"),
        (59, "0m"),
        (60, "1m"),
        (3600, "1h0m"),
        (86400, "1d0h0m"),
    ];
    for &(secs, want) in cases {
        assert_eq!(format_uptime(secs), want, "uptime {secs}s");
    }
}

#[test]
fn format_latency_table_ms_and_s() {
    assert_eq!(format_latency(0.4), "<1ms");
    assert_eq!(format_latency(500.0), "500ms");
    assert_eq!(format_latency(1000.0), "1.0s");
    assert_eq!(format_latency(10_000.0), "10.0s");
}

#[test]
fn format_rate_table_b_k_m_g() {
    assert_eq!(format_rate(100.0), "100B/s");
    assert_eq!(format_rate(1024.0), "1.0K/s");
    assert_eq!(format_rate(1_048_576.0), "1.0M/s");
    assert_eq!(format_rate(1_073_741_824.0), "1.0G/s");
}

#[test]
fn format_bytes_human_boundaries_table() {
    let cases: &[(u64, &str)] = &[
        (1023, "1023B"),
        (1024, "1.0K"),
        (1_048_575, "1024.0K"),
        (1_048_576, "1.0M"),
        (1_073_741_823, "1024.0M"),
        (1_073_741_824, "1.0G"),
    ];
    for &(n, want) in cases {
        assert_eq!(format_bytes(n, UnitMode::Human), want, "human {n}");
    }
}

#[test]
fn format_bytes_gib_mib_bytes_modes() {
    assert_eq!(format_bytes(1_048_576, UnitMode::MiB), "1.0M");
    assert_eq!(format_bytes(1_073_741_824, UnitMode::GiB), "1.0G");
    assert_eq!(format_bytes(255, UnitMode::Bytes), "255B");
}

#[test]
fn format_rate_zero_and_sub_one() {
    assert_eq!(format_rate(0.0), "0B/s");
    assert_eq!(format_rate(0.9), "0B/s");
}
