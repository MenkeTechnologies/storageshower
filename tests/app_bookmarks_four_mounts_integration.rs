//! Four bookmarked mounts preserve bookmark-first ordering with `SortMode::Name`.

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
    app
}

#[test]
fn four_bookmarks_name_order_within_group() {
    let mut app = app_with(vec![disk("/z"), disk("/a"), disk("/m"), disk("/q")]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/m".into(), "/q".into(), "/a".into(), "/z".into()];
    app.update_sorted();
    let m: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(m, vec!["/a", "/m", "/q", "/z"]);
}

#[test]
fn four_bookmarks_pct_sort_within_bookmark_block() {
    let mut app = app_with(vec![disk("/hi"), disk("/lo"), disk("/mid"), disk("/x")]);
    app.prefs.sort_mode = SortMode::Pct;
    app.prefs.bookmarks = vec!["/mid".into(), "/hi".into()];
    for d in &mut app.disks {
        match d.mount.as_str() {
            "/hi" => {
                d.pct = 90.0;
                d.used = 900;
            }
            "/lo" => {
                d.pct = 5.0;
                d.used = 50;
            }
            "/mid" => {
                d.pct = 50.0;
                d.used = 500;
            }
            "/x" => {
                d.pct = 10.0;
                d.used = 100;
            }
            _ => {}
        }
        d.total = 1000;
    }
    app.update_sorted();
    let mounts: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert_eq!(mounts, vec!["/mid", "/hi", "/lo", "/x"]);
}
