//! `tmpfs` mounts appear only when `prefs.show_all` is true (`App::update_sorted`).

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::types::{DiskEntry, SortMode, SysStats};

fn tmpfs_disk(mount: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used: 1,
        total: 1000,
        pct: 0.1,
        kind: DiskKind::Unknown(0),
        fs: "tmpfs".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }
}

fn ext_disk(mount: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used: 1,
        total: 2000,
        pct: 0.05,
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
fn show_all_true_includes_tmpfs() {
    let mut app = app_with(vec![tmpfs_disk("/run/user"), ext_disk("/data")]);
    app.prefs.show_all = true;
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    let mounts: Vec<_> = app
        .sorted_disks()
        .iter()
        .map(|d| d.mount.as_str())
        .collect();
    assert!(mounts.contains(&"/run/user"));
    assert!(mounts.contains(&"/data"));
}

#[test]
fn show_all_false_filters_tmpfs() {
    let mut app = app_with(vec![tmpfs_disk("/run"), ext_disk("/srv")]);
    app.prefs.show_all = false;
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/srv");
}
