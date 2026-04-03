//! `prefs.show_local` retention rules with `DiskKind` and totals (`App::update_sorted`).

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

fn disk(mount: &str, kind: DiskKind, used: u64, total: u64, pct: f64, fs: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used,
        total,
        pct,
        kind,
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
fn show_local_keeps_ssd_and_hdd_with_positive_total() {
    let mut app = app_with(vec![
        disk("/ssd", DiskKind::SSD, 10, 100, 10.0, "apfs"),
        disk("/hdd", DiskKind::HDD, 20, 200, 10.0, "ext4"),
    ]);
    app.prefs.show_local = true;
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 2);
}

#[test]
fn show_local_drops_unknown_kind_zero_total() {
    let mut app = app_with(vec![
        disk("/real", DiskKind::SSD, 1, 100, 1.0, "apfs"),
        disk("/phantom", DiskKind::Unknown(-1), 0, 0, 0.0, "none"),
    ]);
    app.prefs.show_local = true;
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/real");
}

#[test]
fn show_local_keeps_unknown_kind_when_total_positive() {
    let mut app = app_with(vec![disk(
        "/weird",
        DiskKind::Unknown(0),
        50,
        1000,
        5.0,
        "fuse.xyz",
    )]);
    app.prefs.show_local = true;
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
}

#[test]
fn show_local_false_shows_unknown_zero_total() {
    let mut app = app_with(vec![disk(
        "/phantom",
        DiskKind::Unknown(-1),
        0,
        0,
        0.0,
        "none",
    )]);
    app.prefs.show_local = false;
    app.prefs.show_all = true;
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
}

#[test]
fn show_local_true_with_only_unknown_positive_total_sorted_by_name() {
    let mut app = app_with(vec![
        disk("/z", DiskKind::Unknown(-1), 1, 10, 10.0, "fuse"),
        disk("/a", DiskKind::Unknown(-1), 1, 10, 10.0, "fuse"),
    ]);
    app.prefs.show_local = true;
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/a");
    assert_eq!(app.sorted_disks()[1].mount, "/z");
}

#[test]
fn filter_case_insensitive_with_show_local() {
    let mut app = app_with(vec![
        disk("/Volumes/Data", DiskKind::SSD, 1, 100, 1.0, "apfs"),
        disk("/boot", DiskKind::SSD, 1, 100, 1.0, "ext4"),
    ]);
    app.prefs.show_local = true;
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "DATA".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert!(app.sorted_disks()[0].mount.contains("Data"));
}

#[test]
fn filter_unicode_substring() {
    let mut app = app_with(vec![
        disk("/mnt/用户", DiskKind::SSD, 1, 100, 1.0, "apfs"),
        disk("/other", DiskKind::SSD, 1, 100, 1.0, "apfs"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "用户".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/mnt/用户");
}

#[test]
fn clear_filter_restores_full_list() {
    let mut app = app_with(vec![
        disk("/a", DiskKind::SSD, 1, 10, 10.0, "ext4"),
        disk("/b", DiskKind::SSD, 1, 10, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "a".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    app.filter.text.clear();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 2);
}

#[test]
fn filter_no_match_yields_empty() {
    let mut app = app_with(vec![disk("/only", DiskKind::SSD, 1, 10, 10.0, "ext4")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "zzznomatch".into();
    app.update_sorted();
    assert!(app.sorted_disks().is_empty());
}

#[test]
fn show_local_combined_with_show_all_off_filters_tmpfs() {
    let mut app = app_with(vec![
        disk("/real", DiskKind::SSD, 1, 100, 1.0, "apfs"),
        disk("/tmp", DiskKind::Unknown(-1), 1, 100, 1.0, "tmpfs"),
    ]);
    app.prefs.show_local = true;
    app.prefs.show_all = false;
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/real");
}
