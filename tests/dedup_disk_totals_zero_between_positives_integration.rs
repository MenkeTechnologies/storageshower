//! `dedup_disk_totals` with `total == 0` rows mixed with positive totals.

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
fn zero_total_skipped_before_duplicate_positive() {
    let d = vec![disk("/z", 0, 0), disk("/a", 1, 100), disk("/b", 2, 100)];
    assert_eq!(dedup_disk_totals(&d), (100, 1));
}

#[test]
fn zero_between_two_uniques() {
    let d = vec![disk("/z", 0, 0), disk("/a", 3, 30), disk("/b", 4, 40)];
    assert_eq!(dedup_disk_totals(&d), (70, 7));
}
