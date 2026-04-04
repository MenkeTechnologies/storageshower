//! `dedup_disk_totals` order: duplicate `total` rows then a new `total`.

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
fn two_dupes_then_unique_sums_first_and_third() {
    let d = vec![disk("/a", 1, 100), disk("/b", 2, 100), disk("/c", 3, 200)];
    assert_eq!(dedup_disk_totals(&d), (300, 4));
}

#[test]
fn unique_then_dupes_counts_first_occurrence_only() {
    let d = vec![disk("/x", 5, 50), disk("/y", 6, 60), disk("/z", 7, 60)];
    assert_eq!(dedup_disk_totals(&d), (110, 11));
}
