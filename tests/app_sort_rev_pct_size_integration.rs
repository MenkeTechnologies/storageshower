//! `sort_rev` with `SortMode::Pct` and `SortMode::Size` (`App::update_sorted`).

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
fn sort_rev_reverses_pct_order() {
    let mut app = app_with(vec![
        disk("/lo", 1, 100, 10.0, "ext4"),
        disk("/hi", 1, 100, 90.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.sort_rev = true;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/hi");
    assert_eq!(app.sorted_disks()[1].mount, "/lo");
}

#[test]
fn sort_rev_reverses_size_order() {
    let mut app = app_with(vec![
        disk("/small", 1, 1000, 10.0, "ext4"),
        disk("/big", 1, 9000, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.sort_rev = true;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/big");
    assert_eq!(app.sorted_disks()[1].mount, "/small");
}

#[test]
fn pct_sort_stable_when_equal_pct_preserves_input_order() {
    let mut app = app_with(vec![
        disk("/first", 50, 100, 50.0, "ext4"),
        disk("/second", 50, 100, 50.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/first");
    assert_eq!(app.sorted_disks()[1].mount, "/second");
}

#[test]
fn size_sort_stable_when_equal_total_preserves_input_order() {
    let mut app = app_with(vec![
        disk("/aa", 1, 2000, 10.0, "ext4"),
        disk("/bb", 1, 2000, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Size;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/aa");
    assert_eq!(app.sorted_disks()[1].mount, "/bb");
}

#[test]
fn sort_rev_with_bookmarks_pins_before_non_bookmarked() {
    let mut app = app_with(vec![
        disk("/z", 1, 10, 10.0, "ext4"),
        disk("/a", 1, 10, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.sort_rev = true;
    app.prefs.bookmarks = vec!["/a".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/a", "/z"]);
}

#[test]
fn pct_nan_partial_cmp_falls_back_to_equal() {
    let mut app = app_with(vec![
        disk("/nan", 1, 100, f64::NAN, "ext4"),
        disk("/ok", 1, 100, 50.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 2);
}
