//! Filter substring matching on varied mount path shapes (`App::update_sorted`).

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

fn disk(mount: &str, fs: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used: 1,
        total: 100,
        pct: 1.0,
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
fn filter_matches_hyphenated_mount() {
    let mut app = app_with(vec![
        disk("/Volumes/My-Drive", "apfs"),
        disk("/other", "apfs"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "My-Drive".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/Volumes/My-Drive");
}

#[test]
fn filter_matches_dot_in_path() {
    let mut app = app_with(vec![disk("/srv/data.v2", "ext4"), disk("/mnt", "ext4")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = ".v2".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
}

#[test]
fn filter_matches_numeric_segment() {
    let mut app = app_with(vec![
        disk("/disk/by-id/nvme0n1p2", "ext4"),
        disk("/", "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "nvme0".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
}

#[test]
fn filter_matches_underscore() {
    let mut app = app_with(vec![disk("/mnt/backup_drive", "ext4"), disk("/a", "ext4")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "backup_drive".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
}

#[test]
fn filter_single_slash_root() {
    let mut app = app_with(vec![disk("/", "apfs"), disk("/home", "ext4")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "/".into();
    app.update_sorted();
    assert!(app.sorted_disks().iter().any(|d| d.mount == "/"));
}

#[test]
fn filter_lowercase_matches_uppercase_mount() {
    let mut app = app_with(vec![disk("/DATA", "ext4")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "data".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
}

#[test]
fn filter_multiple_matches_keeps_sort_order() {
    let mut app = app_with(vec![
        disk("/app/foo", "ext4"),
        disk("/app/bar", "ext4"),
        disk("/zzz", "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "app".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 2);
    assert_eq!(app.sorted_disks()[0].mount, "/app/bar");
    assert_eq!(app.sorted_disks()[1].mount, "/app/foo");
}

#[test]
fn filter_empty_string_is_noop() {
    let mut app = app_with(vec![disk("/a", "ext4"), disk("/b", "ext4")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = String::new();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 2);
}
