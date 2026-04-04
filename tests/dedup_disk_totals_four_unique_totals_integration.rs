//! `dedup_disk_totals` with four mounts and four distinct `total` values.

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
fn four_distinct_totals_sum_all() {
    let d = vec![
        disk("/a", 10, 100),
        disk("/b", 20, 200),
        disk("/c", 30, 300),
        disk("/d", 40, 400),
    ];
    assert_eq!(dedup_disk_totals(&d), (1000, 100));
}

#[test]
fn two_pairs_same_total_each_once() {
    let d = vec![
        disk("/a", 1, 50),
        disk("/b", 2, 50),
        disk("/c", 3, 60),
        disk("/d", 4, 60),
    ];
    assert_eq!(dedup_disk_totals(&d), (110, 4));
}
