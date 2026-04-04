//! `SortMode::Name` with three bookmarks among four mounts.

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
fn three_bookmarks_before_non_bookmark_name_asc() {
    let mut app = app_with(vec![disk("/a"), disk("/m"), disk("/q"), disk("/z")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/m".into(), "/q".into(), "/z".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/m", "/q", "/z", "/a"]);
}

#[test]
fn three_bookmarks_only_name_asc() {
    let mut app = app_with(vec![disk("/c"), disk("/a"), disk("/b")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/c".into(), "/a".into(), "/b".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/a", "/b", "/c"]);
}
