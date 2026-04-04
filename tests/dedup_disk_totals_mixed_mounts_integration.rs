//! `dedup_disk_totals` with multiple duplicate-total groups (public API).

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
fn two_duplicate_groups_first_used_each() {
    let d = vec![
        disk("/a", 10, 1000),
        disk("/b", 20, 1000),
        disk("/c", 5, 2000),
        disk("/d", 7, 2000),
    ];
    assert_eq!(dedup_disk_totals(&d), (3000, 15));
}

#[test]
fn duplicate_total_order_affects_which_used_counts() {
    let d1 = vec![disk("/x", 1, 500), disk("/y", 2, 500)];
    let d2 = vec![disk("/y", 2, 500), disk("/x", 1, 500)];
    assert_eq!(dedup_disk_totals(&d1), (500, 1));
    assert_eq!(dedup_disk_totals(&d2), (500, 2));
}

#[test]
fn single_duplicate_among_unique_totals() {
    let d = vec![
        disk("/u1", 100, 10_000),
        disk("/u2", 200, 20_000),
        disk("/dup2", 999, 10_000),
    ];
    assert_eq!(dedup_disk_totals(&d), (30_000, 300));
}
