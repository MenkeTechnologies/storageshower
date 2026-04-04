//! `SortMode::Size` with three bookmarks among four mounts.

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

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
fn three_bookmarks_before_non_bookmark_size_asc() {
    let mut app = app_with(vec![
        disk("/a", 1, 100),
        disk("/m", 1, 200),
        disk("/q", 1, 300),
        disk("/z", 1, 400),
    ]);
    app.prefs.sort_mode = SortMode::Size;
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
fn three_bookmarks_only_size_asc() {
    let mut app = app_with(vec![
        disk("/big", 1, 300),
        disk("/mid", 1, 200),
        disk("/small", 1, 100),
    ]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.bookmarks = vec!["/big".into(), "/mid".into(), "/small".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/small", "/mid", "/big"]);
}
