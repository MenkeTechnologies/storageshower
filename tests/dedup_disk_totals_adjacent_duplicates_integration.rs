//! `dedup_disk_totals` with adjacent rows sharing the same `total`.

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
fn two_adjacent_same_total() {
    let d = vec![disk("/a", 3, 777), disk("/b", 9, 777)];
    assert_eq!(dedup_disk_totals(&d), (777, 3));
}

#[test]
fn three_rows_two_totals_middle_dup() {
    let d = vec![disk("/x", 1, 100), disk("/y", 2, 200), disk("/z", 3, 100)];
    assert_eq!(dedup_disk_totals(&d), (300, 3));
}
