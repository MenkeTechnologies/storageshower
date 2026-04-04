//! `SortMode::Size` with two bookmarks among three mounts.

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
fn two_bookmarks_and_one_other_bookmarks_first_by_size() {
    let mut app = app_with(vec![
        disk("/x", 1, 100),
        disk("/y", 1, 200),
        disk("/z", 1, 300),
    ]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.bookmarks = vec!["/y".into(), "/z".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/y", "/z", "/x"]);
}

#[test]
fn both_bookmarks_only_pair_size_asc() {
    let mut app = app_with(vec![disk("/big", 1, 500), disk("/small", 1, 100)]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.bookmarks = vec!["/big".into(), "/small".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/small");
    assert_eq!(app.sorted_disks()[1].mount, "/big");
}
