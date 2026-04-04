//! `dedup_disk_totals` with six mounts and six distinct `total` values.

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
fn six_distinct_totals_sum_all() {
    let d = vec![
        disk("/a", 1, 11),
        disk("/b", 2, 22),
        disk("/c", 3, 33),
        disk("/d", 4, 44),
        disk("/e", 5, 55),
        disk("/f", 6, 66),
    ];
    assert_eq!(dedup_disk_totals(&d), (231, 21));
}
