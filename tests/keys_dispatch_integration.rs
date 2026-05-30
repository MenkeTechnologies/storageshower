//! Round-4 TUI key-dispatch pins: verify slash-opens-filter, q-quits,
//! q-inside-filter is inserted not dispatched, Esc reverts to prev,
//! and Enter commits. These pin the top-level dispatch shape so a
//! refactor cannot accidentally swap "filter active" with "quit" or
//! collapse the filter-mode/main-mode dispatch tables.

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::prefs::Prefs;
use storageshower::types::{DiskEntry, SysStats};

fn make_key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn test_disks() -> Vec<DiskEntry> {
    vec![DiskEntry {
        mount: "/".into(),
        used: 50_000_000_000,
        total: 100_000_000_000,
        pct: 50.0,
        kind: DiskKind::SSD,
        fs: "apfs".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }]
}

fn test_app() -> App {
    let stats = SysStats::default();
    let disks = test_disks();
    let shared = Arc::new(Mutex::new((stats.clone(), disks.clone())));
    let mut app = App::new_default(shared);
    app.disks = disks;
    app.stats = stats;
    app.prefs = Prefs::default();
    app.test_mode = true;
    app.update_sorted();
    app
}

#[test]
fn slash_opens_filter_mode_from_top_level() {
    let mut app = test_app();
    assert!(!app.filter.active, "filter must start inactive");
    app.handle_key(make_key(KeyCode::Char('/')));
    assert!(app.filter.active, "/ should activate filter mode");
    assert!(!app.quit, "/ must not also set quit");
}

#[test]
fn q_quits_from_top_level_without_touching_filter() {
    let mut app = test_app();
    app.handle_key(make_key(KeyCode::Char('q')));
    assert!(app.quit, "q must set quit");
    assert!(!app.filter.active, "q must not also activate filter");
}

#[test]
fn esc_in_filter_mode_reverts_buffer_to_prev() {
    let mut app = test_app();
    app.handle_key(make_key(KeyCode::Char('/')));
    assert!(app.filter.active);
    app.handle_key(make_key(KeyCode::Char('a')));
    app.handle_key(make_key(KeyCode::Char('b')));
    app.handle_key(make_key(KeyCode::Char('c')));
    assert_eq!(app.filter.text, "abc");
    let prev = app.filter.prev.clone();
    app.handle_key(make_key(KeyCode::Esc));
    assert!(!app.filter.active, "Esc must deactivate filter");
    assert_eq!(app.filter.text, prev, "Esc must revert text to prev");
    assert_eq!(app.filter.cursor, 0, "Esc must reset cursor to 0");
}

#[test]
fn enter_in_filter_mode_commits_buffer_keeps_text() {
    let mut app = test_app();
    app.handle_key(make_key(KeyCode::Char('/')));
    app.handle_key(make_key(KeyCode::Char('x')));
    app.handle_key(make_key(KeyCode::Char('y')));
    assert_eq!(app.filter.text, "xy");
    app.handle_key(make_key(KeyCode::Enter));
    assert!(!app.filter.active, "Enter must deactivate filter");
    assert_eq!(app.filter.text, "xy", "Enter must keep typed text");
}

#[test]
fn q_inside_filter_mode_is_inserted_not_dispatched_as_quit() {
    let mut app = test_app();
    app.handle_key(make_key(KeyCode::Char('/')));
    assert!(app.filter.active);
    app.handle_key(make_key(KeyCode::Char('q')));
    assert!(!app.quit, "q in filter mode must not set quit");
    assert_eq!(app.filter.text, "q", "q in filter mode must be inserted");
}
