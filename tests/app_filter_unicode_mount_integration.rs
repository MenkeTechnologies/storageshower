//! Filter substring on Unicode mount paths (`App::update_sorted`).

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
fn filter_matches_unicode_substring() {
    let mut app = app_with(vec![disk("/mnt/other"), disk("/mnt/日本語/data")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "日本".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/mnt/日本語/data");
}

#[test]
fn filter_case_folds_unicode_query() {
    let mut app = app_with(vec![disk("/store/Äpfel")]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "äpfel".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
}
