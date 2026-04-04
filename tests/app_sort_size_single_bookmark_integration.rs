//! `SortMode::Size` with a single bookmark: bookmarked mount sorts before larger non-bookmark.

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
fn bookmark_smaller_total_sorts_first() {
    let mut app = app_with(vec![disk("/small", 1, 100), disk("/big", 1, 900)]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.bookmarks = vec!["/small".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/small");
    assert_eq!(app.sorted_disks()[1].mount, "/big");
}

#[test]
fn bookmark_larger_total_still_first_in_list() {
    let mut app = app_with(vec![disk("/a", 1, 50), disk("/b", 1, 500)]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.bookmarks = vec!["/b".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/b");
    assert_eq!(app.sorted_disks()[1].mount, "/a");
}
