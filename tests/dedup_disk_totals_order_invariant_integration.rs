//! `dedup_disk_totals` is invariant to slice order for the same multiset of rows.

use storageshower::system::dedup_disk_totals;
use storageshower::types::DiskEntry;
use sysinfo::DiskKind;

fn d(mount: &str, used: u64, total: u64) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used,
        total,
        pct: 0.0,
        kind: DiskKind::SSD,
        fs: "x".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }
}

#[test]
fn reorder_same_three_mounts_same_result() {
    let a = vec![d("/a", 1, 100), d("/b", 2, 100), d("/c", 3, 200)];
    let b = vec![d("/c", 3, 200), d("/a", 1, 100), d("/b", 2, 100)];
    assert_eq!(dedup_disk_totals(&a), dedup_disk_totals(&b));
}

#[test]
fn duplicate_total_storage_invariant_but_used_follows_first_row() {
    let first_x = vec![d("/x", 10, 500), d("/y", 99, 500)];
    let first_y = vec![d("/y", 99, 500), d("/x", 10, 500)];
    let ax = dedup_disk_totals(&first_x);
    let ay = dedup_disk_totals(&first_y);
    assert_eq!(ax.0, ay.0);
    assert_eq!(ax.0, 500);
    assert_ne!(ax.1, ay.1);
    assert_eq!(ax.1, 10);
    assert_eq!(ay.1, 99);
}

#[test]
fn four_entries_two_unique_totals_permutation() {
    let v1 = vec![
        d("/1", 1, 1000),
        d("/2", 2, 1000),
        d("/3", 3, 2000),
        d("/4", 4, 2000),
    ];
    let v2 = vec![
        d("/3", 3, 2000),
        d("/1", 1, 1000),
        d("/4", 4, 2000),
        d("/2", 2, 1000),
    ];
    assert_eq!(dedup_disk_totals(&v1), dedup_disk_totals(&v2));
}
