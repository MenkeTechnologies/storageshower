//! `SortMode::Name` with four bookmarks among four mounts.

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

fn disk(mount: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used: 1,
        total: 100,
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
fn four_bookmarks_all_mounts_name_asc_within_block() {
    let mut app = app_with(vec![disk("/z"), disk("/a"), disk("/m"), disk("/q")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/z".into(), "/a".into(), "/m".into(), "/q".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/a", "/m", "/q", "/z"]);
}

#[test]
fn three_bookmarks_one_non_bookmark_name_asc() {
    let mut app = app_with(vec![disk("/a"), disk("/b"), disk("/c"), disk("/z")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/b".into(), "/c".into(), "/z".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/b", "/c", "/z", "/a"]);
}
