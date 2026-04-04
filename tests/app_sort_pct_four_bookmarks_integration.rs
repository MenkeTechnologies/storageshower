//! `SortMode::Pct` with four bookmarks among four mounts.

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
    app.prefs.show_all = true;
    app
}

#[test]
fn four_bookmarks_all_mounts_pct_asc_within_block() {
    let mut app = app_with(vec![
        disk("/z", 900, 1000, 90.0),
        disk("/a", 100, 1000, 10.0),
        disk("/m", 500, 1000, 50.0),
        disk("/q", 300, 1000, 30.0),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/z".into(), "/a".into(), "/m".into(), "/q".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/a", "/q", "/m", "/z"]);
}

#[test]
fn three_bookmarks_one_non_bookmark_pct_asc() {
    let mut app = app_with(vec![
        disk("/a", 100, 1000, 10.0),
        disk("/b", 200, 1000, 20.0),
        disk("/c", 500, 1000, 50.0),
        disk("/d", 900, 1000, 90.0),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/b".into(), "/c".into(), "/d".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/b", "/c", "/d", "/a"]);
}
