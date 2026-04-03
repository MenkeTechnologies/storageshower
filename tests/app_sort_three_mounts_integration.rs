//! Three-mount `App::update_sorted` cases for each `SortMode`.

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

fn disk(mount: &str, used: u64, total: u64, pct: f64) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used,
        total,
        pct,
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
    app
}

#[test]
fn three_by_name_asc() {
    let mut app = app_with(vec![
        disk("/z", 1, 10, 10.0),
        disk("/a", 1, 10, 10.0),
        disk("/m", 1, 10, 10.0),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/a", "/m", "/z"]);
}

#[test]
fn three_by_pct_asc() {
    let mut app = app_with(vec![
        disk("/hi", 90, 100, 90.0),
        disk("/lo", 5, 100, 5.0),
        disk("/mid", 50, 100, 50.0),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/lo");
    assert_eq!(app.sorted_disks()[2].mount, "/hi");
}

#[test]
fn three_by_size_asc() {
    let mut app = app_with(vec![
        disk("/big", 1, 3000, 1.0),
        disk("/tiny", 1, 10, 1.0),
        disk("/mid", 1, 1000, 1.0),
    ]);
    app.prefs.sort_mode = SortMode::Size;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/tiny");
    assert_eq!(app.sorted_disks()[2].mount, "/big");
}

#[test]
fn three_by_name_desc_rev() {
    let mut app = app_with(vec![
        disk("/a", 1, 10, 10.0),
        disk("/b", 1, 10, 10.0),
        disk("/c", 1, 10, 10.0),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.sort_rev = true;
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/c", "/b", "/a"]);
}
