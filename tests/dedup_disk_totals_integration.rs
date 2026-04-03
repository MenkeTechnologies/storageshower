//! Integration tests for `storageshower::system::dedup_disk_totals` (public API).

use storageshower::system::dedup_disk_totals;
use storageshower::types::{DiskEntry, SmartHealth};
use sysinfo::DiskKind;

fn disk(mount: &str, used: u64, total: u64, pct: f64, fs: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used,
        total,
        pct,
        kind: DiskKind::SSD,
        fs: fs.into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }
}

#[test]
fn empty_slice_returns_zeros() {
    assert_eq!(dedup_disk_totals(&[]), (0, 0));
}

#[test]
fn single_mount_sums_normally() {
    let d = vec![disk("/a", 10, 100, 10.0, "ext4")];
    assert_eq!(dedup_disk_totals(&d), (100, 10));
}

#[test]
fn identical_total_counts_storage_once_uses_first_used_only() {
    let d = vec![
        disk("/a", 100, 1000, 10.0, "apfs"),
        disk("/b", 200, 1000, 20.0, "apfs"),
    ];
    assert_eq!(dedup_disk_totals(&d), (1000, 100));
}

#[test]
fn three_mounts_same_total_first_used_wins() {
    let d = vec![
        disk("/1", 5, 2000, 0.0, "x"),
        disk("/2", 99, 2000, 0.0, "x"),
        disk("/3", 1, 2000, 0.0, "x"),
    ];
    assert_eq!(dedup_disk_totals(&d), (2000, 5));
}

#[test]
fn distinct_totals_all_counted() {
    let d = vec![
        disk("/a", 10, 100, 10.0, "ext4"),
        disk("/b", 20, 200, 10.0, "ext4"),
    ];
    assert_eq!(dedup_disk_totals(&d), (300, 30));
}

#[test]
fn zero_total_entries_skipped_entirely() {
    let d = vec![
        disk("/empty", 0, 0, 0.0, "tmpfs"),
        disk("/real", 50, 500, 10.0, "ext4"),
    ];
    assert_eq!(dedup_disk_totals(&d), (500, 50));
}

#[test]
fn only_zero_totals_yield_zero() {
    let d = vec![
        disk("/a", 0, 0, 0.0, "tmpfs"),
        disk("/b", 0, 0, 0.0, "tmpfs"),
    ];
    assert_eq!(dedup_disk_totals(&d), (0, 0));
}

#[test]
fn large_u64_totals_no_panic() {
    let t = u64::MAX / 4;
    let d = vec![disk("/big", 1, t, 0.0, "zfs")];
    assert_eq!(dedup_disk_totals(&d), (t, 1));
}

#[test]
fn dedup_preserves_order_independent_summing_for_unique_totals() {
    let d = vec![
        disk("/z", 1, 300, 0.0, "x"),
        disk("/y", 2, 200, 0.0, "x"),
        disk("/x", 4, 100, 0.0, "x"),
    ];
    assert_eq!(dedup_disk_totals(&d), (600, 7));
}

#[test]
fn extra_fields_on_disk_entry_do_not_affect_dedup() {
    let mut a = disk("/a", 10, 1000, 1.0, "apfs");
    a.latency_ms = Some(12.0);
    a.io_read_rate = Some(100.0);
    a.smart_status = Some(SmartHealth::Verified);
    let mut b = disk("/b", 20, 1000, 2.0, "apfs");
    b.latency_ms = Some(50.0);
    assert_eq!(dedup_disk_totals(&[a, b]), (1000, 10));
}

#[test]
fn hdd_and_ssd_kinds_mixed_same_total_still_dedup() {
    let mut a = disk("/ssd", 5, 500, 1.0, "apfs");
    a.kind = DiskKind::SSD;
    let mut b = disk("/hdd", 9, 500, 1.0, "apfs");
    b.kind = DiskKind::HDD;
    assert_eq!(dedup_disk_totals(&[a, b]), (500, 5));
}
