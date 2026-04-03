//! Bookmarks combined with `sort_rev` (`App::update_sorted`).

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
    app
}

#[test]
fn bookmark_still_first_when_name_sort_reversed() {
    let mut app = app_with(vec![disk("/a"), disk("/b"), disk("/c")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.sort_rev = true;
    app.prefs.bookmarks = vec!["/b".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/b");
}

#[test]
fn sort_rev_name_z_before_a_without_bookmark() {
    let mut app = app_with(vec![disk("/a"), disk("/z")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.sort_rev = true;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/z");
}

#[test]
fn bookmark_plus_sort_rev_pct() {
    let mut app = app_with(vec![disk("/low"), disk("/hi"), disk("/mid")]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.sort_rev = true;
    app.prefs.bookmarks = vec!["/mid".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/mid");
}

#[test]
fn two_bookmarks_sort_rev_size() {
    let mut app = app_with(vec![disk("/s"), disk("/m"), disk("/l")]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.sort_rev = true;
    app.prefs.bookmarks = vec!["/m".into(), "/s".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m[0], "/m");
    assert_eq!(m[1], "/s");
}

#[test]
fn bookmark_only_mount_survives_filter_with_sort_rev() {
    let mut app = app_with(vec![disk("/keep-me"), disk("/drop")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.sort_rev = true;
    app.prefs.bookmarks = vec!["/keep-me".into()];
    app.filter.text = "keep".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/keep-me");
}
