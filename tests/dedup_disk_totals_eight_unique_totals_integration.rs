//! `dedup_disk_totals` with eight mounts and eight distinct `total` values.

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
fn eight_distinct_totals_sum_all() {
    let d = vec![
        disk("/a", 1, 10),
        disk("/b", 2, 20),
        disk("/c", 3, 30),
        disk("/d", 4, 40),
        disk("/e", 5, 50),
        disk("/f", 6, 60),
        disk("/g", 7, 70),
        disk("/h", 8, 80),
    ];
    assert_eq!(dedup_disk_totals(&d), (360, 36));
}
