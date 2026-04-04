//! `dedup_disk_totals` with a single mount (public API).

use storageshower::system::dedup_disk_totals;
use storageshower::types::DiskEntry;
use sysinfo::DiskKind;

#[test]
fn one_large_mount() {
    let d = vec![DiskEntry {
        mount: "/data".into(),
        used: 500_000_000_000,
        total: 1_000_000_000_000,
        pct: 50.0,
        kind: DiskKind::SSD,
        fs: "ext4".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }];
    assert_eq!(dedup_disk_totals(&d), (1_000_000_000_000, 500_000_000_000));
}
