//! Bookmarks with `SortMode::{Name,Pct,Size}` and `sort_rev` (`App::update_sorted`).

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

fn disk(mount: &str, used: u64, total: u64, pct: f64, fs: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used,
        total,
        pct,
        kind: DiskKind::SSD,
        fs: fs.into(),
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
fn bookmark_first_with_sort_pct_ascending() {
    let mut app = app_with(vec![
        disk("/low", 10, 100, 10.0, "ext4"),
        disk("/hi", 80, 100, 80.0, "ext4"),
        disk("/mid", 50, 100, 50.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/mid".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/mid");
}

#[test]
fn bookmark_first_with_sort_size_ascending() {
    let mut app = app_with(vec![
        disk("/tiny", 1, 100, 1.0, "ext4"),
        disk("/huge", 1, 10_000, 1.0, "ext4"),
        disk("/mark", 1, 500, 1.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.bookmarks = vec!["/mark".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/mark");
}

#[test]
fn two_bookmarks_both_before_non_bookmarked_pct_sort() {
    let mut app = app_with(vec![
        disk("/a", 1, 100, 5.0, "ext4"),
        disk("/b", 1, 100, 95.0, "ext4"),
        disk("/bm1", 1, 100, 50.0, "ext4"),
        disk("/bm2", 1, 100, 51.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/bm2".into(), "/bm1".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m[0], "/bm1");
    assert_eq!(m[1], "/bm2");
}

#[test]
fn bookmark_with_sort_rev_name() {
    let mut app = app_with(vec![
        disk("/y", 1, 10, 10.0, "ext4"),
        disk("/x", 1, 10, 10.0, "ext4"),
        disk("/z", 1, 10, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.sort_rev = true;
    app.prefs.bookmarks = vec!["/x".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/x");
}

#[test]
fn bookmark_not_matching_filter_excluded() {
    let mut app = app_with(vec![
        disk("/data", 1, 10, 10.0, "ext4"),
        disk("/home", 1, 10, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/data".into()];
    app.filter.text = "home".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/home");
}

#[test]
fn empty_bookmarks_behaves_like_plain_sort_pct() {
    let mut app = app_with(vec![
        disk("/lo", 1, 100, 5.0, "ext4"),
        disk("/hi", 1, 100, 99.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec![];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/lo");
    assert_eq!(app.sorted_disks()[1].mount, "/hi");
}
