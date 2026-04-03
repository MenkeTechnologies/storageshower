//! `App::update_sorted` / `sorted_disks` and column width helpers via the public `app` re-exports.
#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use sysinfo::DiskKind;

use storageshower::app::{App, mount_col_width, right_col_width, right_col_width_static};
use storageshower::prefs::Prefs;
use storageshower::types::{DiskEntry, SmartHealth, SortMode, SysStats, UnitMode};

fn disk(mount: &str, used: u64, total: u64, pct: f64, fs: &str) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used,
        total,
        pct,
        kind: DiskKind::SSD,
        fs: fs.into(),
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
fn sorted_by_name_ascending() {
    let mut app = app_with(vec![
        disk("/zzz", 1, 2, 50.0, "ext4"),
        disk("/aaa", 1, 2, 50.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/aaa");
    assert_eq!(app.sorted_disks()[1].mount, "/zzz");
}

#[test]
fn sorted_by_size_ascending() {
    let mut app = app_with(vec![
        disk("/big", 1, 9_000, 10.0, "ext4"),
        disk("/small", 1, 1_000, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Size;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/small");
    assert_eq!(app.sorted_disks()[1].mount, "/big");
}

#[test]
fn sorted_by_pct_ascending() {
    let mut app = app_with(vec![
        disk("/hi", 1, 100, 90.0, "ext4"),
        disk("/lo", 1, 100, 10.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Pct;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/lo");
    assert_eq!(app.sorted_disks()[1].mount, "/hi");
}

#[test]
fn sort_rev_reverses_name() {
    let mut app = app_with(vec![
        disk("/a", 1, 2, 50.0, "ext4"),
        disk("/b", 1, 2, 50.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.sort_rev = true;
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/b");
    assert_eq!(app.sorted_disks()[1].mount, "/a");
}

#[test]
fn bookmarks_prioritize_mount() {
    let mut app = app_with(vec![
        disk("/first", 1, 2, 50.0, "ext4"),
        disk("/bookmark", 1, 2, 50.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/bookmark".into()];
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, "/bookmark");
}

#[test]
fn filter_narrows_sorted_list() {
    let mut app = app_with(vec![
        disk("/data", 1, 2, 50.0, "ext4"),
        disk("/home", 1, 2, 50.0, "ext4"),
    ]);
    app.prefs.sort_mode = SortMode::Name;
    app.filter.text = "home".into();
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/home");
}

#[test]
fn mount_col_width_default_inner_120() {
    let p = Prefs::default();
    assert_eq!(mount_col_width(120, &p), 40);
}

#[test]
fn right_col_width_static_default() {
    let p = Prefs::default();
    assert_eq!(right_col_width_static(&p), 22);
}

#[test]
fn right_col_width_static_no_used() {
    let mut p = Prefs::default();
    p.show_used = false;
    assert_eq!(right_col_width_static(&p), 7);
}

#[test]
fn right_col_width_dynamic_bytes_wider_than_human() {
    let mut app = app_with(vec![disk(
        "/",
        10_000_000_000,
        20_000_000_000,
        50.0,
        "apfs",
    )]);
    app.prefs.unit_mode = UnitMode::Bytes;
    app.prefs.col_bar_end_w = 0;
    let w_bytes = right_col_width(&app);
    app.prefs.unit_mode = UnitMode::Human;
    let w_human = right_col_width(&app);
    assert!(w_bytes >= w_human);
}

#[test]
fn sorted_cache_empty_when_no_disks() {
    let mut app = app_with(vec![]);
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert!(app.sorted_disks().is_empty());
}

#[test]
fn tmpfs_filtered_when_show_all_false() {
    let mut app = app_with(vec![
        disk("/real", 1, 100, 50.0, "ext4"),
        disk("/tmp", 1, 100, 50.0, "tmpfs"),
    ]);
    app.prefs.show_all = false;
    app.prefs.sort_mode = SortMode::Name;
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
    assert_eq!(app.sorted_disks()[0].mount, "/real");
}

#[test]
fn smart_status_preserved_in_sorted_cache() {
    let mut d = disk("/x", 1, 2, 50.0, "ext4");
    d.smart_status = Some(SmartHealth::Failing);
    let mut app = app_with(vec![d]);
    app.update_sorted();
    assert_eq!(
        app.sorted_disks()[0].smart_status,
        Some(SmartHealth::Failing)
    );
}
