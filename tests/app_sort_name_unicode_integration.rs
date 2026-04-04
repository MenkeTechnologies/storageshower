//! `SortMode::Name` with Unicode mount paths.

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
fn hiragana_names_lexicographic_bytes() {
    let mut app = app_with(vec![disk("/vol/い"), disk("/vol/あ")]);
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/vol/あ");
    assert_eq!(app.sorted_disks()[1].mount, "/vol/い");
}

#[test]
fn mixed_scripts_sort_by_mount_string() {
    let mut app = app_with(vec![disk("/z"), disk("/α")]);
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    let a = app.sorted_disks()[0].mount.as_str();
    let b = app.sorted_disks()[1].mount.as_str();
    assert!(a < b, "expected strict order, got {a:?} {b:?}");
}
