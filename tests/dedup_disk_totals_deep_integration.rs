//! Extra `dedup_disk_totals` cases (rates, SMART, latency fields must not affect math).
#![allow(clippy::too_many_arguments)]

use storageshower::system::dedup_disk_totals;
use storageshower::types::{DiskEntry, SmartHealth};
use sysinfo::DiskKind;

fn disk_full(
    mount: &str,
    used: u64,
    total: u64,
    pct: f64,
    fs: &str,
    latency_ms: Option<f64>,
    io_read: Option<f64>,
    io_write: Option<f64>,
    smart: Option<SmartHealth>,
) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used,
        total,
        pct,
        kind: DiskKind::SSD,
        fs: fs.into(),
        latency_ms,
        io_read_rate: io_read,
        io_write_rate: io_write,
        smart_status: smart,
    }
}

#[test]
fn dedup_ignores_identical_totals_four_mounts() {
    let d = vec![
        disk_full("/a", 1, 500, 0.0, "x", None, None, None, None),
        disk_full("/b", 2, 500, 0.0, "x", None, None, None, None),
        disk_full("/c", 3, 500, 0.0, "x", None, None, None, None),
        disk_full("/d", 100, 2000, 0.0, "y", None, None, None, None),
    ];
    assert_eq!(dedup_disk_totals(&d), (2500, 101));
}

#[test]
fn dedup_with_latency_and_io_on_duplicate_total() {
    let d = vec![
        disk_full(
            "/a",
            10,
            1000,
            1.0,
            "nfs",
            Some(12.0),
            Some(100.0),
            Some(200.0),
            None,
        ),
        disk_full(
            "/b",
            99,
            1000,
            9.0,
            "nfs",
            Some(50.0),
            None,
            None,
            Some(SmartHealth::Verified),
        ),
    ];
    assert_eq!(dedup_disk_totals(&d), (1000, 10));
}

#[test]
fn dedup_single_entry_max_u64_total() {
    let d = vec![disk_full(
        "/big",
        u64::MAX / 2,
        u64::MAX / 2,
        50.0,
        "zfs",
        None,
        None,
        None,
        None,
    )];
    let (t, u) = dedup_disk_totals(&d);
    assert_eq!(t, u64::MAX / 2);
    assert_eq!(u, u64::MAX / 2);
}

#[test]
fn dedup_two_unique_totals_large() {
    let d = vec![
        disk_full("/a", 1, 9_999_999_999_999, 0.0, "x", None, None, None, None),
        disk_full("/b", 2, 8_888_888_888_888, 0.0, "y", None, None, None, None),
    ];
    assert_eq!(
        dedup_disk_totals(&d),
        (9_999_999_999_999 + 8_888_888_888_888, 3)
    );
}

#[test]
fn dedup_mixed_hdd_ssd_same_total() {
    let mut a = disk_full("/ssd", 5, 400, 0.0, "apfs", None, None, None, None);
    a.kind = DiskKind::SSD;
    let mut b = disk_full("/hdd", 9, 400, 0.0, "ext4", None, None, None, None);
    b.kind = DiskKind::HDD;
    assert_eq!(dedup_disk_totals(&[a, b]), (400, 5));
}

#[test]
fn dedup_zero_total_skipped_even_with_io() {
    let d = vec![disk_full(
        "/ghost",
        0,
        0,
        0.0,
        "tmpfs",
        None,
        Some(1.0e9),
        Some(1.0e9),
        None,
    )];
    assert_eq!(dedup_disk_totals(&d), (0, 0));
}
