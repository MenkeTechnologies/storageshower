//! `right_col_width` honors `col_bar_end_w` when set (integration path).

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::columns::right_col_width;
use storageshower::types::{DiskEntry, SysStats};

fn disk(mount: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used: 10,
        total: 100,
        pct: 10.0,
        kind: DiskKind::SSD,
        fs: "ext4".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }
}

#[test]
fn col_bar_end_w_returns_max_with_min() {
    let stats = SysStats::default();
    let disks = vec![disk("/a")];
    let shared = Arc::new(Mutex::new((stats.clone(), disks.clone())));
    let mut app = App::new_default(shared);
    app.disks = disks;
    app.stats = stats;
    app.test_mode = true;
    app.prefs.col_bar_end_w = 42;
    app.prefs.show_used = true;
    assert_eq!(right_col_width(&app), 42);
}

#[test]
fn col_bar_end_w_below_min_clamps_to_five() {
    let stats = SysStats::default();
    let shared = Arc::new(Mutex::new((stats.clone(), vec![])));
    let mut app = App::new_default(shared);
    app.disks = vec![];
    app.stats = stats;
    app.test_mode = true;
    app.prefs.col_bar_end_w = 2;
    assert_eq!(right_col_width(&app), 5);
}
