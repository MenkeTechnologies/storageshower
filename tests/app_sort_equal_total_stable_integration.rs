//! `SortMode::Size` with identical `total` preserves relative order (stable sort).

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

fn disk(mount: &str, total: u64) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used: 1,
        total,
        pct: 1.0,
        kind: DiskKind::SSD,
        fs: "ext4".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }
}

fn app_with(disks: Vec<DiskEntry>) -> App {
    let stats = SysStats::default();
    let shared = Arc::new(Mutex::new((stats.clone(), disks.clone())));
    let mut app = App::new_default(shared);
    app.disks = disks;
    app.stats = stats;
    app.test_mode = true;
    app.prefs.show_all = true;
    app
}

#[test]
fn equal_total_keeps_insertion_order_asc() {
    let mut app = app_with(vec![disk("/z", 500), disk("/a", 500)]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.sort_rev = false;
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/z", "/a"]);
}

#[test]
fn equal_total_rev_reverses_slice() {
    let mut app = app_with(vec![disk("/first", 100), disk("/second", 100)]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.sort_rev = true;
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/second", "/first"]);
}
