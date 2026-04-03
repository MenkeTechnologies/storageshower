//! Filter substring + bookmark pin ordering together (`App::update_sorted`).

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
fn filter_narrows_then_bookmarked_mount_leads_then_name_order() {
    let mut app = app_with(vec![
        disk("/alpha", 1, 10, 10.0, "ext4"),
        disk("/beta", 1, 10, 10.0, "ext4"),
        disk("/gamma", 1, 10, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/gamma".into()];
    app.filter.text = "a".into();
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    // Bookmarked /gamma first, then remaining matches in name order.
    assert_eq!(m, vec!["/gamma", "/alpha", "/beta"]);
}

#[test]
fn bookmarked_row_first_when_both_match_filter() {
    let mut app = app_with(vec![
        disk("/data", 1, 10, 10.0, "ext4"),
        disk("/home", 1, 10, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/home".into()];
    app.filter.text = "home".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/home");
}

#[test]
fn bookmark_not_in_filtered_set_does_not_panic() {
    let mut app = app_with(vec![
        disk("/only", 1, 10, 10.0, "ext4"),
        disk("/other", 1, 10, 10.0, "ext4"),
    ]);
    app.prefs.bookmarks = vec!["/missing-bookmark".into()];
    app.filter.text = "only".into();
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/only");
}

#[test]
fn two_bookmarks_stable_order_among_filtered() {
    let mut app = app_with(vec![
        disk("/a1", 1, 10, 10.0, "ext4"),
        disk("/a2", 1, 10, 10.0, "ext4"),
        disk("/z9", 1, 10, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/z9".into(), "/a1".into()];
    app.filter.text = "a".into();
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/a1", "/a2"]);
    assert_eq!(m[0], "/a1");
}

#[test]
fn filter_empty_bookmark_order_follows_name_sort_then_pins() {
    let mut app = app_with(vec![
        disk("/m", 1, 10, 10.0, "ext4"),
        disk("/n", 1, 10, 10.0, "ext4"),
        disk("/o", 1, 10, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/o".into(), "/m".into()];
    app.filter.text = String::new();
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/m", "/o", "/n"]);
}
