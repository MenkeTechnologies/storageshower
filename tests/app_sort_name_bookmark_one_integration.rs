//! `SortMode::Name` with a single bookmark: bookmarked mount lists first.

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
fn bookmark_later_alphabetically_sorts_first() {
    let mut app = app_with(vec![disk("/z"), disk("/a")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/z".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/z");
    assert_eq!(app.sorted_disks()[1].mount, "/a");
}

#[test]
fn bookmark_earlier_alphabetically_still_first() {
    let mut app = app_with(vec![disk("/z"), disk("/a")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/a".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/a");
    assert_eq!(app.sorted_disks()[1].mount, "/z");
}
