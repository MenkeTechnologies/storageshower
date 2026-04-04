//! `right_col_width` respects a wide `col_pct_w` (public `columns` + `app`).

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
fn wide_pct_column_increases_right_width() {
    let stats = SysStats::default();
    let disks = vec![disk("/a")];
    let shared = Arc::new(Mutex::new((stats.clone(), disks.clone())));
    let mut app = App::new_default(shared);
    app.disks = disks;
    app.stats = stats;
    app.test_mode = true;
    app.prefs.col_bar_end_w = 0;
    app.prefs.show_used = true;
    app.prefs.col_pct_w = 0;
    let w_default_pct = right_col_width(&app);
    app.prefs.col_pct_w = 25;
    let w_wide_pct = right_col_width(&app);
    assert!(
        w_wide_pct > w_default_pct,
        "wide_pct={w_wide_pct} default={w_default_pct}"
    );
}

#[test]
fn wide_pct_still_at_least_floor() {
    let stats = SysStats::default();
    let shared = Arc::new(Mutex::new((stats.clone(), vec![])));
    let mut app = App::new_default(shared);
    app.disks = vec![];
    app.stats = stats;
    app.test_mode = true;
    app.prefs.col_bar_end_w = 0;
    app.prefs.show_used = true;
    app.prefs.col_pct_w = 30;
    assert!(right_col_width(&app) >= 22);
}
