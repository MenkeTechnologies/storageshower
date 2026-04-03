//! `App::update_sorted` filter: case-insensitive substring on mount path.

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
fn filter_matches_mount_case_insensitively() {
    let mut app = app_with(vec![disk("/DATA"), disk("/other")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "data".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/DATA");
}

#[test]
fn filter_uppercase_query_matches_lowercase_mount() {
    let mut app = app_with(vec![disk("/var"), disk("/var/log")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "LOG".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/var/log");
}

#[test]
fn clearing_filter_restores_full_list() {
    let mut app = app_with(vec![disk("/a"), disk("/b")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "zzz".into();
    app.update_sorted();
    assert!(app.sorted_disks().is_empty());
    app.filter.text.clear();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 2);
}

#[test]
fn filter_with_no_matches_yields_empty_sorted_cache() {
    let mut app = app_with(vec![disk("/one"), disk("/two")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "does-not-exist".into();
    app.update_sorted();
    assert!(app.sorted_disks().is_empty());
}
