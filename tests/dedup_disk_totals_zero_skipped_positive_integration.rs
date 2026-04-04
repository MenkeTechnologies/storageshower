//! `dedup_disk_totals` skips `total == 0` rows before summing real totals.

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
fn leading_zero_total_skipped_then_duplicate_total() {
    let d = vec![
        disk("/z", 0, 0),
        disk("/a", 100, 1000),
        disk("/b", 200, 1000),
    ];
    assert_eq!(dedup_disk_totals(&d), (1000, 100));
}

#[test]
fn only_positive_totals_after_zeros() {
    let d = vec![disk("/t", 0, 0), disk("/u", 5, 500)];
    assert_eq!(dedup_disk_totals(&d), (500, 5));
}
