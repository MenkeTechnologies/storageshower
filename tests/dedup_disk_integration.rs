//! Integration tests for `system::dedup_disk_totals` (APFS-style shared totals).

use storageshower::system::dedup_disk_totals;
use storageshower::types::DiskEntry;
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
fn empty_slice() {
    assert_eq!(dedup_disk_totals(&[]), (0, 0));
}

#[test]
fn one_disk() {
    let disks = [disk("/a", 10, 100, 10.0, "ext4")];
    assert_eq!(dedup_disk_totals(&disks), (100, 10));
}

#[test]
fn two_unique_totals_sum_both() {
    let disks = [
        disk("/a", 1, 100, 1.0, "ext4"),
        disk("/b", 2, 200, 1.0, "ext4"),
    ];
    assert_eq!(dedup_disk_totals(&disks), (300, 3));
}

#[test]
fn two_same_total_counts_storage_once() {
    let disks = [
        disk("/snap1", 50, 1000, 5.0, "ext4"),
        disk("/snap2", 60, 1000, 6.0, "ext4"),
    ];
    let (tot, used) = dedup_disk_totals(&disks);
    assert_eq!(tot, 1000);
    assert_eq!(used, 50);
}

#[test]
fn three_disks_two_share_total() {
    let disks = [
        disk("/a", 100, 1000, 10.0, "apfs"),
        disk("/b", 200, 1000, 20.0, "apfs"),
        disk("/c", 50, 500, 10.0, "ext4"),
    ];
    let (tot, used) = dedup_disk_totals(&disks);
    assert_eq!(tot, 1500);
    assert_eq!(used, 150);
}

#[test]
fn zero_total_skipped() {
    let disks = [disk("/z", 0, 0, 0.0, "ext4")];
    assert_eq!(dedup_disk_totals(&disks), (0, 0));
}

#[test]
fn zero_total_mixed_with_positive() {
    let disks = [
        disk("/z", 0, 0, 0.0, "ext4"),
        disk("/a", 7, 700, 1.0, "ext4"),
    ];
    assert_eq!(dedup_disk_totals(&disks), (700, 7));
}

#[test]
fn four_mounts_two_pairs_same_total() {
    let disks = [
        disk("/m1", 10, 3000, 1.0, "ext4"),
        disk("/m2", 20, 3000, 1.0, "ext4"),
        disk("/m3", 5, 4000, 1.0, "ext4"),
        disk("/m4", 5, 4000, 1.0, "ext4"),
    ];
    let (tot, used) = dedup_disk_totals(&disks);
    assert_eq!(tot, 7000);
    assert_eq!(used, 15);
}

#[test]
fn large_u64_totals() {
    let t = 9_223_372_036_854_775_000u64;
    let disks = [disk("/big", 1, t, 0.0, "xfs")];
    assert_eq!(dedup_disk_totals(&disks), (t, 1));
}
