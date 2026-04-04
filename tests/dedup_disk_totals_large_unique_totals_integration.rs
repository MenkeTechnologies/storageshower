//! `dedup_disk_totals` with large distinct `total` values (no duplicates).

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
fn two_large_distinct_totals_sum_both() {
    let t1 = 9_000_000_000_000u64;
    let t2 = 8_000_000_000_000u64;
    let d = vec![disk("/a", t1 / 10, t1), disk("/b", t2 / 4, t2)];
    assert_eq!(dedup_disk_totals(&d), (t1 + t2, t1 / 10 + t2 / 4));
}

#[test]
fn three_mounts_unique_totals_all_counted() {
    let d = vec![
        disk("/x", 1, 100_000_000_000),
        disk("/y", 2, 200_000_000_000),
        disk("/z", 3, 300_000_000_000),
    ];
    assert_eq!(dedup_disk_totals(&d), (600_000_000_000, 6));
}
