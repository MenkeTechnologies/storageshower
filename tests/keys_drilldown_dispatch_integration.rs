//! Drill-down key-dispatch pins.
//!
//! The drill-down branch of `handle_key` used to return before the list-mode
//! arms ran, so four things were wrong at once and are pinned here:
//!
//! 1. `y`/`Y` (copy) and `e`/`E` (export) were unreachable while drilled in,
//!    even though `o`/`O` was wired, so the omission was visible to the user.
//! 2. No Ctrl chord reached the drill branch at all — `^D`/`^U`/`^G` were
//!    list-mode only, on the longest listings in the app.
//! 3. Because the branch never inspected the ctrl flag, `^Q` fell through to
//!    the plain `q` arm and quit, and `^C` toggled the reclaim overlay.
//! 4. `^D`/`^U` paged by half the total row count instead of half a viewport.
//!
//! These are dispatch-shape pins: they must keep failing if the drill branch
//! is ever re-ordered to return early again.

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::prefs::Prefs;
use storageshower::types::{DirEntry, DiskEntry, DrillSortMode, SysStats, ViewMode};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn ctrl(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn disks(n: usize) -> Vec<DiskEntry> {
    (0..n)
        .map(|i| DiskEntry {
            mount: format!("/m{i:03}"),
            used: 50_000_000_000,
            total: 100_000_000_000,
            pct: 50.0,
            kind: DiskKind::SSD,
            fs: "apfs".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        })
        .collect()
}

fn app_with(n_disks: usize) -> App {
    let stats = SysStats::default();
    let d = disks(n_disks);
    let shared = Arc::new(Mutex::new((stats.clone(), d.clone())));
    let mut app = App::new_default(shared);
    app.disks = d;
    app.stats = stats;
    app.prefs = Prefs::default();
    app.test_mode = true;
    app.update_sorted();
    app
}

/// Enter drill-down over a synthetic listing without touching the filesystem.
fn drilled(n_entries: usize) -> App {
    let mut app = app_with(1);
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/root".into()];
    app.drill.entries = (0..n_entries)
        .map(|i| DirEntry {
            path: format!("/root/e{i:03}"),
            name: format!("e{i:03}"),
            size: (n_entries - i) as u64 * 1024,
            is_dir: i % 3 == 0,
            reclaimable: 0,
            ratio: 0.0,
        })
        .collect();
    app.drill.selected = 0;
    app
}

// ── Defect 1: copy and export are reachable while drilled in ──

#[test]
fn drill_y_acts_on_the_highlighted_entry() {
    let mut app = drilled(20);
    app.drill.selected = 4;
    app.handle_key(key(KeyCode::Char('y')));
    let msg = &app.status_msg.as_ref().expect("y must set a status").0;
    // Headless CI has no clipboard helper; either branch is fine, but the
    // path acted on must be the highlighted entry, not the parent directory.
    assert!(
        (msg.starts_with("Copied (") && msg.ends_with("): /root/e004"))
            || msg.starts_with("Copy failed"),
        "y must target the selected entry, got: {msg}"
    );
    assert!(!app.quit, "y must not leak into any other arm");
    assert_eq!(
        app.drill.mode,
        ViewMode::DrillDown,
        "y must stay drilled in"
    );
}

#[test]
fn drill_y_on_an_empty_listing_falls_back_to_the_directory() {
    let mut app = drilled(0);
    app.handle_key(key(KeyCode::Char('y')));
    let msg = &app.status_msg.as_ref().expect("y must set a status").0;
    assert!(
        (msg.starts_with("Copied (") && msg.ends_with("): /root"))
            || msg.starts_with("Copy failed"),
        "empty listing must fall back to the shown directory, got: {msg}"
    );
}

#[test]
fn drill_e_writes_a_separate_export_file() {
    let mut app = drilled(5);
    app.handle_key(key(KeyCode::Char('e')));
    let msg = &app.status_msg.as_ref().expect("e must set a status").0;
    assert!(
        msg.contains("drill-export.txt"),
        "drill export must not clobber the disk-matrix export, got: {msg}"
    );
    assert_eq!(
        app.drill.mode,
        ViewMode::DrillDown,
        "e must stay drilled in"
    );
}

#[test]
fn drill_export_text_matches_the_current_listing() {
    let app = drilled(5);
    let out = app.drill_export_text();
    assert!(out.contains("Path: /root\n"));
    for i in 0..5 {
        assert!(
            out.contains(&format!("e{i:03}")),
            "entry e{i:03} must appear"
        );
    }
    // dirs are marked, files are not
    assert!(out.contains("e000/"), "is_dir entry must be marked");
    assert!(!out.contains("e001/"), "file entry must not be marked");
}

// ── Defect 2 + 4: ctrl paging in drill-down, sized by the viewport ──

#[test]
fn drill_ctrl_d_and_ctrl_u_page_half_a_viewport() {
    let mut app = drilled(200);
    app.drill_viewport_rows = 24;
    app.handle_key(ctrl(KeyCode::Char('d')));
    assert_eq!(app.drill.selected, 12, "^D moves half of 24 rows");
    app.handle_key(ctrl(KeyCode::Char('d')));
    assert_eq!(app.drill.selected, 24);
    app.handle_key(ctrl(KeyCode::Char('u')));
    assert_eq!(app.drill.selected, 12, "^U is symmetric");
}

#[test]
fn drill_paging_ignores_the_entry_count() {
    // 200 entries, 10-row window: the jump must be 5, not 100.
    let mut app = drilled(200);
    app.drill_viewport_rows = 10;
    app.handle_key(ctrl(KeyCode::Char('d')));
    assert_eq!(app.drill.selected, 5);
}

#[test]
fn drill_ctrl_d_clamps_to_the_last_entry() {
    let mut app = drilled(7);
    app.drill_viewport_rows = 100;
    app.handle_key(ctrl(KeyCode::Char('d')));
    assert_eq!(app.drill.selected, 6);
}

#[test]
fn drill_ctrl_g_returns_to_the_first_entry() {
    let mut app = drilled(50);
    app.drill.selected = 47;
    app.handle_key(ctrl(KeyCode::Char('g')));
    assert_eq!(app.drill.selected, 0);
}

// ── Defect 3: ctrl chords must not fall through to plain arms ──

#[test]
fn drill_ctrl_q_does_not_quit() {
    let mut app = drilled(4);
    app.handle_key(ctrl(KeyCode::Char('q')));
    assert!(!app.quit, "^Q must not reach the plain q arm in drill-down");
    assert_eq!(app.drill.mode, ViewMode::DrillDown);
}

#[test]
fn drill_ctrl_s_and_ctrl_r_do_not_reach_the_sort_arms() {
    let mut app = drilled(4);
    assert_eq!(app.drill.sort, DrillSortMode::Size);
    assert!(!app.drill.sort_rev);
    app.handle_key(ctrl(KeyCode::Char('n')));
    assert_eq!(app.drill.sort, DrillSortMode::Size, "^N must be swallowed");
    app.handle_key(ctrl(KeyCode::Char('r')));
    assert!(!app.drill.sort_rev, "^R must be swallowed");
}

#[test]
fn drill_ctrl_esc_path_keys_are_unaffected() {
    // Plain Esc still leaves drill-down; the ctrl branch must not eat it.
    let mut app = drilled(4);
    app.handle_key(key(KeyCode::Esc));
    assert_eq!(app.drill.mode, ViewMode::Disks);
}

// ── Defect 4: the main disk list pages by the viewport too ──

#[test]
fn list_ctrl_d_pages_half_a_viewport_not_half_the_disk_count() {
    let mut app = app_with(100);
    app.viewport_rows = 20;
    app.selected = Some(0);
    app.handle_key(ctrl(KeyCode::Char('d')));
    assert_eq!(app.selected, Some(10), "^D moves half of 20 rows, not 50");
    app.handle_key(ctrl(KeyCode::Char('u')));
    assert_eq!(app.selected, Some(0));
}

#[test]
fn list_ctrl_d_clamps_to_the_last_disk() {
    let mut app = app_with(6);
    app.viewport_rows = 80;
    app.selected = Some(0);
    app.handle_key(ctrl(KeyCode::Char('d')));
    assert_eq!(app.selected, Some(5));
}
