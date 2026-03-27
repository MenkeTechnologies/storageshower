use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use storageshower::app::{mount_col_width, right_col_width, right_col_width_static, App};
use storageshower::helpers::{format_bytes, format_uptime, truncate_mount};
use storageshower::prefs::Prefs;
use storageshower::system::{chrono_now, collect_disk_entries, collect_sys_stats};
use storageshower::types::*;

use sysinfo::{DiskKind, System};

// ─── Test helpers ──────────────────────────────────────────────────────────

fn make_key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn sample_disks() -> Vec<DiskEntry> {
    vec![
        DiskEntry { mount: "/".into(), used: 50_000_000_000, total: 100_000_000_000, pct: 50.0, kind: DiskKind::SSD, fs: "apfs".into() },
        DiskEntry { mount: "/data".into(), used: 900_000_000_000, total: 1_000_000_000_000, pct: 90.0, kind: DiskKind::HDD, fs: "xfs".into() },
        DiskEntry { mount: "/home".into(), used: 80_000_000_000, total: 200_000_000_000, pct: 40.0, kind: DiskKind::SSD, fs: "ext4".into() },
    ]
}

fn make_app_with_disks(disks: Vec<DiskEntry>) -> App {
    let stats = SysStats::default();
    let shared = Arc::new(Mutex::new((stats.clone(), disks.clone())));
    let mut app = App::new_default(shared);
    app.disks = disks;
    app.stats = stats;
    app.prefs = Prefs::default();
    app
}

// ─── End-to-end: sort → filter → navigate ──────────────────────────────────

#[test]
fn full_workflow_sort_filter_navigate() {
    let mut app = make_app_with_disks(sample_disks());

    // Sort by size
    app.handle_key(make_key(KeyCode::Char('s')));
    assert_eq!(app.prefs.sort_mode, SortMode::Size);
    let disks = app.sorted_disks();
    assert!(disks[0].total <= disks[1].total);

    // Filter to only root
    app.handle_key(make_key(KeyCode::Char('/')));
    assert!(app.filter_mode);
    // Type "/"  — but "/" also triggers filter mode, so we need a different char
    app.handle_key(make_key(KeyCode::Char('d')));
    app.handle_key(make_key(KeyCode::Char('a')));
    app.handle_key(make_key(KeyCode::Char('t')));
    app.handle_key(make_key(KeyCode::Char('a')));
    app.handle_key(make_key(KeyCode::Enter));
    assert!(!app.filter_mode);
    assert_eq!(app.filter, "data");

    let filtered = app.sorted_disks();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].mount, "/data");

    // Navigate: select first
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.selected, Some(0));

    // Clear filter
    app.handle_key(make_key(KeyCode::Char('0')));
    assert!(app.filter.is_empty());
    assert_eq!(app.sorted_disks().len(), 3);
}

// ─── Prefs round-trip ──────────────────────────────────────────────────────

#[test]
fn prefs_serialize_deserialize_all_fields() {
    let mut p = Prefs::default();
    p.sort_mode = SortMode::Pct;
    p.sort_rev = true;
    p.bar_style = BarStyle::Thin;
    p.color_mode = ColorMode::Blue;
    p.unit_mode = UnitMode::MiB;
    p.col_mount_w = 30;
    p.col_bar_end_w = 25;
    p.col_pct_w = 8;
    p.thresh_warn = 60;
    p.thresh_crit = 85;
    p.refresh_rate = 5;
    p.compact = true;
    p.full_mount = true;
    p.show_bars = false;
    p.show_border = false;
    p.show_header = false;
    p.show_used = false;
    p.show_local = true;
    p.show_all = false;

    let toml_str = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&toml_str).unwrap();

    assert_eq!(q.sort_mode, SortMode::Pct);
    assert!(q.sort_rev);
    assert_eq!(q.bar_style, BarStyle::Thin);
    assert_eq!(q.color_mode, ColorMode::Blue);
    assert_eq!(q.unit_mode, UnitMode::MiB);
    assert_eq!(q.col_mount_w, 30);
    assert_eq!(q.col_bar_end_w, 25);
    assert_eq!(q.col_pct_w, 8);
    assert_eq!(q.thresh_warn, 60);
    assert_eq!(q.thresh_crit, 85);
    assert_eq!(q.refresh_rate, 5);
    assert!(q.compact);
    assert!(q.full_mount);
    assert!(!q.show_bars);
    assert!(!q.show_border);
    assert!(!q.show_header);
    assert!(!q.show_used);
    assert!(q.show_local);
    assert!(!q.show_all);
}

// ─── System data collection ────────────────────────────────────────────────

#[test]
fn system_collects_real_disks() {
    let disks = collect_disk_entries();
    assert!(!disks.is_empty());
    // Root mount should exist on unix
    #[cfg(unix)]
    assert!(disks.iter().any(|d| d.mount == "/"),
        "Expected root mount '/' in disk entries");
}

#[test]
fn system_stats_populated() {
    let sys = System::new_all();
    let stats = collect_sys_stats(&sys);
    assert!(!stats.hostname.is_empty());
    assert!(stats.cpu_count > 0);
    assert!(stats.mem_total > 0);
}

#[test]
fn chrono_now_date_and_time() {
    let (d, t) = chrono_now();
    // Validate lengths
    assert_eq!(d.len(), 10);
    assert_eq!(t.len(), 8);
    // Parseable year
    let year: u32 = d[..4].parse().unwrap();
    assert!(year >= 2024);
}

// ─── Helpers ───────────────────────────────────────────────────────────────

#[test]
fn format_bytes_boundary_values() {
    assert_eq!(format_bytes(1023, UnitMode::Human), "1023B");
    assert_eq!(format_bytes(1024, UnitMode::Human), "1.0K");
    assert_eq!(format_bytes(1_048_575, UnitMode::Human), "1024.0K");
    assert_eq!(format_bytes(1_048_576, UnitMode::Human), "1.0M");
}

#[test]
fn format_uptime_boundary() {
    assert_eq!(format_uptime(3599), "59m");
    assert_eq!(format_uptime(3600), "1h0m");
    assert_eq!(format_uptime(86399), "23h59m");
    assert_eq!(format_uptime(86400), "1d0h0m");
}

#[test]
fn truncate_mount_various_widths() {
    let long = "/very/long/mount/point/path";
    for w in 5..30 {
        let r = truncate_mount(long, w);
        assert!(r.chars().count() <= w,
            "truncate_mount({}, {}) produced {} chars", long, w, r.chars().count());
    }
}

// ─── Column widths ─────────────────────────────────────────────────────────

#[test]
fn column_widths_consistent() {
    let mut p = Prefs::default();
    // With defaults, right_col_width_static should match expectations
    assert!(right_col_width_static(&p) >= 7);
    p.show_used = false;
    assert_eq!(right_col_width_static(&p), 7);
    p.show_used = true;
    assert_eq!(right_col_width_static(&p), 22);

    // mount_col_width
    let mw = mount_col_width(100, &p);
    assert!(mw >= 12);
    assert!(mw <= 100);
}

#[test]
fn right_col_width_dynamic_with_data() {
    let mut app = make_app_with_disks(sample_disks());
    let w = right_col_width(&app);
    assert!(w >= 22);

    // With custom override
    app.prefs.col_bar_end_w = 30;
    assert_eq!(right_col_width(&app), 30);
}

// ─── App navigation with real disk data ────────────────────────────────────

#[test]
fn navigate_through_real_disks() {
    let disks = collect_disk_entries();
    if disks.is_empty() { return; }

    let mut app = make_app_with_disks(disks);
    let count = app.sorted_disks().len();

    // Select first
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.selected, Some(0));

    // Navigate to last
    app.handle_key(make_key(KeyCode::End));
    assert_eq!(app.selected, Some(count - 1));

    // Deselect
    app.handle_key(make_key(KeyCode::Esc));
    assert_eq!(app.selected, None);
}

// ─── Full display cycle ────────────────────────────────────────────────────

#[test]
fn cycle_all_display_options() {
    let mut app = make_app_with_disks(sample_disks());

    // Cycle bar styles
    for _ in 0..4 {
        app.handle_key(make_key(KeyCode::Char('b')));
    }
    assert_eq!(app.prefs.bar_style, BarStyle::Gradient); // back to start

    // Cycle color modes
    for _ in 0..4 {
        app.handle_key(make_key(KeyCode::Char('c')));
    }
    assert_eq!(app.prefs.color_mode, ColorMode::Default);

    // Cycle unit modes
    for _ in 0..4 {
        app.handle_key(make_key(KeyCode::Char('i')));
    }
    assert_eq!(app.prefs.unit_mode, UnitMode::Human);

    // Cycle refresh rates
    for _ in 0..4 {
        app.handle_key(make_key(KeyCode::Char('f')));
    }
    assert_eq!(app.prefs.refresh_rate, 1);
}

// ─── Filter with vim-style editing ─────────────────────────────────────────

#[test]
fn filter_vim_editing_workflow() {
    let mut app = make_app_with_disks(sample_disks());

    // Enter filter mode
    app.handle_key(make_key(KeyCode::Char('/')));
    assert!(app.filter_mode);

    // Type "home"
    for c in "home".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    assert_eq!(app.filter_buf, "home");
    assert_eq!(app.filter_cursor, 4);

    // Live filter should show 1 result
    assert_eq!(app.sorted_disks().len(), 1);

    // Move cursor to start (Ctrl+A)
    app.handle_key(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter_cursor, 0);

    // Kill to end (Ctrl+K)
    app.handle_key(KeyEvent {
        code: KeyCode::Char('k'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter_buf, "");
    // Live filter: all disks visible again
    assert_eq!(app.sorted_disks().len(), 3);

    // Esc cancels — but since we entered with empty filter_prev, it restores ""
    app.handle_key(make_key(KeyCode::Esc));
    assert!(!app.filter_mode);
}

// ─── Sort click simulation ─────────────────────────────────────────────────

#[test]
fn sort_mode_toggle_via_keys() {
    let mut app = make_app_with_disks(sample_disks());

    // Start from size sort so we can test switching
    app.prefs.sort_mode = SortMode::Size;
    app.prefs.sort_rev = false;

    // Switch to name
    app.handle_key(make_key(KeyCode::Char('n')));
    assert_eq!(app.prefs.sort_mode, SortMode::Name);
    assert!(!app.prefs.sort_rev);

    // Press again → reverse
    app.handle_key(make_key(KeyCode::Char('n')));
    assert_eq!(app.prefs.sort_mode, SortMode::Name);
    assert!(app.prefs.sort_rev);

    // Switch to pct
    app.handle_key(make_key(KeyCode::Char('u')));
    assert_eq!(app.prefs.sort_mode, SortMode::Pct);
    assert!(!app.prefs.sort_rev); // reset on mode change
}

// ─── Shared state refresh ──────────────────────────────────────────────────

#[test]
fn refresh_data_updates_from_shared() {
    let stats = SysStats::default();
    let disks = sample_disks();
    let shared = Arc::new(Mutex::new((stats, disks)));
    let mut app = App::new_default(Arc::clone(&shared));

    // Modify shared state
    let new_disk = DiskEntry {
        mount: "/new".into(),
        used: 1000,
        total: 2000,
        pct: 50.0,
        kind: DiskKind::SSD,
        fs: "ext4".into(),
    };
    {
        let mut lock = shared.lock().unwrap();
        lock.1 = vec![new_disk];
        lock.0.hostname = "testhost".into();
    }

    app.refresh_data();
    assert_eq!(app.disks.len(), 1);
    assert_eq!(app.disks[0].mount, "/new");
    assert_eq!(app.stats.hostname, "testhost");
}

#[test]
fn refresh_data_blocked_when_paused() {
    let stats = SysStats::default();
    let disks = sample_disks();
    let shared = Arc::new(Mutex::new((stats, disks)));
    let mut app = App::new_default(Arc::clone(&shared));
    let original_count = app.disks.len();

    app.paused = true;

    // Modify shared state
    {
        let mut lock = shared.lock().unwrap();
        lock.1 = vec![];
    }

    app.refresh_data();
    // Should NOT have updated
    assert_eq!(app.disks.len(), original_count);
}
