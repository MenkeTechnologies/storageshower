//! `dedup_disk_totals` with seventeen mounts and seventeen distinct `total` values.

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
fn seventeen_distinct_totals_sum_all() {
    let d = vec![
        disk("/a", 1, 10),
        disk("/b", 2, 20),
        disk("/c", 3, 30),
        disk("/d", 4, 40),
        disk("/e", 5, 50),
        disk("/f", 6, 60),
        disk("/g", 7, 70),
        disk("/h", 8, 80),
        disk("/i", 9, 90),
        disk("/j", 10, 100),
        disk("/k", 11, 110),
        disk("/l", 12, 120),
        disk("/m", 13, 130),
        disk("/n", 14, 140),
        disk("/o", 15, 150),
        disk("/p", 16, 160),
        disk("/q", 17, 170),
    ];
    assert_eq!(dedup_disk_totals(&d), (1530, 153));
}
