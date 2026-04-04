//! `SortMode::Size` with identical `total`: stable sort preserves input order (asc).

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

fn disk(mount: &str, used: u64, total: u64, pct: f64) -> DiskEntry {
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
    app.prefs.show_all = true;
    app
}

#[test]
fn same_total_asc_preserves_input_order() {
    let mut app = app_with(vec![
        disk("/first", 100, 1000, 10.0),
        disk("/second", 500, 1000, 50.0),
    ]);
    app.prefs.sort_mode = SortMode::Size;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/first");
    assert_eq!(app.sorted_disks()[1].mount, "/second");
}

#[test]
fn same_total_desc_rev_reverses_relative_order() {
    let mut app = app_with(vec![
        disk("/first", 100, 1000, 10.0),
        disk("/second", 500, 1000, 50.0),
    ]);
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.sort_rev = true;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/second");
    assert_eq!(app.sorted_disks()[1].mount, "/first");
}
