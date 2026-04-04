//! `SortMode::Pct` with a single bookmark: bookmarked mount lists before non-bookmark regardless of %.

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
fn bookmark_lower_pct_sorts_before_higher_non_bookmark() {
    let mut app = app_with(vec![
        disk("/hi", 900, 1000, 90.0),
        disk("/lo", 100, 1000, 10.0),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/lo".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/lo");
    assert_eq!(app.sorted_disks()[1].mount, "/hi");
}

#[test]
fn bookmark_higher_pct_lists_first() {
    let mut app = app_with(vec![
        disk("/hi", 900, 1000, 90.0),
        disk("/lo", 100, 1000, 10.0),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/hi".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/hi");
    assert_eq!(app.sorted_disks()[1].mount, "/lo");
}
