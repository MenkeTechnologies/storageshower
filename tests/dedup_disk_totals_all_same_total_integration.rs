//! `dedup_disk_totals` when every mount shares the same `total` (only first `used` counts).

use storageshower::system::dedup_disk_totals;
use storageshower::types::DiskEntry;
use sysinfo::DiskKind;

fn disk(mount: &str, used: u64, total: u64) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used,
        total,
        pct: 0.0,
        kind: DiskKind::SSD,
        fs: "ext4".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }
}

#[test]
fn three_mounts_same_total_counts_storage_once() {
    let d = vec![
        disk("/first", 11, 5000),
        disk("/second", 22, 5000),
        disk("/third", 33, 5000),
    ];
    assert_eq!(dedup_disk_totals(&d), (5000, 11));
}

#[test]
fn four_mounts_same_total_first_used_only() {
    let d = vec![
        disk("/a", 7, 100),
        disk("/b", 8, 100),
        disk("/c", 9, 100),
        disk("/d", 1, 100),
    ];
    assert_eq!(dedup_disk_totals(&d), (100, 7));
}
