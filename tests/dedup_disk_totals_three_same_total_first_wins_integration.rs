//! `dedup_disk_totals` when three mounts share the same `total` (first `used` wins).

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
fn three_same_total_counts_first_used_only() {
    let d = vec![
        disk("/a", 10, 500),
        disk("/b", 20, 500),
        disk("/c", 30, 500),
    ];
    assert_eq!(dedup_disk_totals(&d), (500, 10));
}

#[test]
fn two_same_total_then_unique() {
    let d = vec![disk("/x", 5, 100), disk("/y", 6, 100), disk("/z", 7, 200)];
    assert_eq!(dedup_disk_totals(&d), (300, 12));
}
