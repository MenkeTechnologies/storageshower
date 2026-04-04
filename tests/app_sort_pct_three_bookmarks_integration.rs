//! `SortMode::Pct` with three bookmarks among four mounts.

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
fn three_bookmarks_before_non_bookmark_pct_asc() {
    let mut app = app_with(vec![
        disk("/a", 100, 1000, 10.0),
        disk("/m", 300, 1000, 30.0),
        disk("/q", 500, 1000, 50.0),
        disk("/z", 900, 1000, 90.0),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/m".into(), "/q".into(), "/z".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/m", "/q", "/z", "/a"]);
}

#[test]
fn three_bookmarks_only_pct_asc() {
    let mut app = app_with(vec![
        disk("/hi", 800, 1000, 80.0),
        disk("/lo", 100, 1000, 10.0),
        disk("/mid", 400, 1000, 40.0),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/hi".into(), "/lo".into(), "/mid".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/lo", "/mid", "/hi"]);
}
