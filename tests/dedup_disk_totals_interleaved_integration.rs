//! `dedup_disk_totals` with interleaved duplicate totals (first `used` wins per `total`).

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
fn two_hundred_between_hundreds() {
    let d = vec![disk("/a", 1, 100), disk("/b", 5, 200), disk("/c", 9, 100)];
    assert_eq!(dedup_disk_totals(&d), (300, 6));
}

#[test]
fn duplicate_then_unique_then_duplicate() {
    let d = vec![
        disk("/x", 10, 1000),
        disk("/y", 20, 1000),
        disk("/z", 3, 500),
        disk("/w", 7, 500),
    ];
    assert_eq!(dedup_disk_totals(&d), (1500, 13));
}
