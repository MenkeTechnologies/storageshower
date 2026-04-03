//! Bookmark ordering after primary sort (`App::update_sorted`). Exercises stable `sort_by_key` with bookmark priority.

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

fn disk(mount: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used: 1,
        total: 2,
        pct: 50.0,
        kind: DiskKind::SSD,
        fs: "ext4".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }
}

fn disk_pct(mount: &str, pct: f64) -> DiskEntry {
    let total = 1000u64;
    let used = ((total as f64) * (pct / 100.0)) as u64;
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
    app
}

#[test]
fn bookmark_only_middle_mount_comes_first_after_name_sort() {
    let mut app = app_with(vec![disk("/zebra"), disk("/alpha"), disk("/mid")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/mid".into()];
    app.update_sorted();
    assert_eq!(
        app.sorted_disks()
            .iter()
            .map(|d| d.mount.as_str())
            .collect::<Vec<_>>(),
        vec!["/mid", "/alpha", "/zebra"]
    );
}

#[test]
fn two_bookmarked_mounts_stay_before_unbookmarked_preserving_name_order() {
    let mut app = app_with(vec![disk("/z"), disk("/a"), disk("/m")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/m".into(), "/a".into()];
    app.update_sorted();
    assert_eq!(
        app.sorted_disks()
            .iter()
            .map(|d| d.mount.as_str())
            .collect::<Vec<_>>(),
        vec!["/a", "/m", "/z"]
    );
}

#[test]
fn bookmark_with_filter_keeps_only_matching_and_still_prioritizes_bookmark() {
    let mut app = app_with(vec![disk("/data"), disk("/home"), disk("/home/extra")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/home/extra".into()];
    app.filter.text = "home".into();
    app.update_sorted();
    let names: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(names, vec!["/home/extra", "/home"]);
}

#[test]
fn bookmark_sort_runs_after_pct_sort() {
    let mut app = app_with(vec![disk_pct("/hi", 90.0), disk_pct("/low", 10.0)]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/low".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/low");
    assert_eq!(app.sorted_disks()[1].mount, "/hi");
}
