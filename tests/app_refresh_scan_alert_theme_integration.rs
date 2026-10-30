//! `refresh_data` side paths and theme-chooser state round-trips.
//!
//! Pins three things the rest of the suite does not reach:
//!
//! 1. A finished background drill scan is collected by `refresh_data` and
//!    sorted with the current drill sort, including while the disk view is
//!    paused (pausing freezes disk stats, not the directory scanner).
//! 2. Threshold alerts fire on the exact warn boundary, re-arm after a mount
//!    drops back below it, and name only mounts that newly crossed.
//! 3. The theme chooser pre-selects an active custom theme and Esc restores
//!    the custom theme, not just the built-in color mode.

use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use sysinfo::DiskKind;

use storageshower::app::App;
use storageshower::prefs::Prefs;
use storageshower::types::{
    ColorMode, DirEntry, DiskEntry, DrillSortMode, SysStats, ThemeColors, ViewMode,
};

type Shared = Arc<Mutex<(SysStats, Vec<DiskEntry>)>>;

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn disk(mount: &str, pct: f64) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used: pct as u64,
        total: 100,
        pct,
        kind: DiskKind::SSD,
        fs: "apfs".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }
}

fn dir(name: &str, size: u64) -> DirEntry {
    DirEntry {
        path: format!("/root/{name}"),
        name: name.into(),
        size,
        is_dir: true,
        reclaimable: 0,
        ratio: 0.0,
    }
}

/// App over `disks` with warn at 70%, saving disabled.
fn app_with(disks: Vec<DiskEntry>) -> (App, Shared) {
    let shared: Shared = Arc::new(Mutex::new((SysStats::default(), disks)));
    let mut app = App::new_default(Arc::clone(&shared));
    app.prefs = Prefs::default();
    app.prefs.thresh_warn = 70;
    app.test_mode = true;
    (app, shared)
}

fn set_disks(shared: &Shared, disks: Vec<DiskEntry>) {
    shared.lock().unwrap().1 = disks;
}

fn alert_msg(app: &App) -> String {
    app.status_msg
        .as_ref()
        .map(|(m, _)| m.clone())
        .unwrap_or_default()
}

// ── 1. Drill scan pickup ──

#[test]
fn refresh_collects_finished_scan_and_applies_current_sort() {
    let (mut app, _shared) = app_with(vec![disk("/", 10.0)]);
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/root".into()];
    app.drill.sort = DrillSortMode::Name;
    app.drill.sort_rev = false;
    app.drill.selected = 5;
    app.drill.scroll_offset = 3;
    app.drill.scanning = true;
    *app.drill.scan_result.lock().unwrap() =
        Some(vec![dir("Zeta", 1), dir("alpha", 3), dir("Mid", 2)]);

    app.refresh_data();

    assert!(!app.drill.scanning);
    assert!(
        app.drill.scan_result.lock().unwrap().is_none(),
        "result slot must be drained so the next scan is not shadowed"
    );
    let names: Vec<&str> = app.drill.entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["alpha", "Mid", "Zeta"]);
    assert_eq!(app.drill.selected, 0);
    assert_eq!(app.drill.scroll_offset, 0);
}

#[test]
fn refresh_collects_scan_while_paused_but_keeps_disks_frozen() {
    let (mut app, shared) = app_with(vec![disk("/", 10.0)]);
    app.paused = true;
    app.drill.scanning = true;
    *app.drill.scan_result.lock().unwrap() = Some(vec![dir("small", 1), dir("big", 9)]);
    set_disks(&shared, vec![disk("/a", 10.0), disk("/b", 10.0)]);

    app.refresh_data();

    assert!(!app.drill.scanning);
    let names: Vec<&str> = app.drill.entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["big", "small"], "default drill sort is size desc");
    assert_eq!(
        app.disks.len(),
        1,
        "paused view must not take new disk stats"
    );
    assert_eq!(app.disks[0].mount, "/");
}

#[test]
fn refresh_with_scan_still_running_leaves_listing_untouched() {
    let (mut app, _shared) = app_with(vec![disk("/", 10.0)]);
    app.drill.entries = vec![dir("prev", 7)];
    app.drill.scanning = true;

    app.refresh_data();

    assert!(app.drill.scanning);
    assert_eq!(app.drill.entries.len(), 1);
    assert_eq!(app.drill.entries[0].name, "prev");
}

#[test]
fn refresh_ignores_stale_result_when_not_scanning() {
    let (mut app, _shared) = app_with(vec![disk("/", 10.0)]);
    app.drill.entries = vec![dir("current", 7)];
    *app.drill.scan_result.lock().unwrap() = Some(vec![dir("stale", 1)]);

    app.refresh_data();

    assert_eq!(app.drill.entries[0].name, "current");
    assert!(app.drill.scan_result.lock().unwrap().is_some());
}

// ── 2. Threshold alerts ──

#[test]
fn alert_fires_at_exact_warn_boundary_but_not_just_below() {
    let (mut app, shared) = app_with(vec![disk("/", 69.9)]);
    app.refresh_data();
    assert!(app.alert.flash.is_none());
    assert!(app.alert.mounts.is_empty());

    set_disks(&shared, vec![disk("/", 70.0)]);
    app.refresh_data();
    assert!(app.alert.flash.is_some(), "pct == warn must alert");
    assert!(app.alert.mounts.contains("/"));
}

#[test]
fn alert_rearms_after_mount_drops_below_warn() {
    let (mut app, shared) = app_with(vec![disk("/", 80.0)]);
    app.refresh_data();
    assert!(app.alert.flash.is_some());

    set_disks(&shared, vec![disk("/", 50.0)]);
    app.alert.flash = None;
    app.refresh_data();
    assert!(app.alert.flash.is_none());
    assert!(
        !app.alert.mounts.contains("/"),
        "recovered mount must leave the alert set"
    );

    set_disks(&shared, vec![disk("/", 85.0)]);
    app.refresh_data();
    assert!(
        app.alert.flash.is_some(),
        "second crossing must alert again"
    );
    assert!(alert_msg(&app).contains("/ 85%"));
}

#[test]
fn alert_message_names_only_newly_crossed_mounts() {
    let (mut app, shared) = app_with(vec![disk("/old", 90.0), disk("/new", 10.0)]);
    app.refresh_data();
    assert!(alert_msg(&app).contains("/old 90%"));

    app.status_msg = None;
    set_disks(&shared, vec![disk("/old", 91.0), disk("/new", 75.0)]);
    app.refresh_data();

    let msg = alert_msg(&app);
    assert!(msg.contains("/new 75%"), "got: {msg}");
    assert!(
        !msg.contains("/old"),
        "already-alerted mount repeated: {msg}"
    );
    assert!(app.alert.mounts.contains("/old"));
    assert!(app.alert.mounts.contains("/new"));
}

#[test]
fn alert_message_joins_simultaneous_crossings_in_disk_order() {
    let (mut app, shared) = app_with(vec![disk("/a", 10.0), disk("/b", 10.0)]);
    app.refresh_data();
    set_disks(&shared, vec![disk("/a", 71.0), disk("/b", 99.0)]);
    app.refresh_data();
    assert!(alert_msg(&app).ends_with("ALERT: /a 71%, /b 99%"));
}

// ── 3. Theme chooser with custom themes ──

fn custom(n: u8) -> ThemeColors {
    ThemeColors {
        blue: n,
        green: n + 1,
        purple: n + 2,
        light_purple: n + 3,
        royal: n + 4,
        dark_purple: n + 5,
    }
}

#[test]
fn chooser_preselects_active_custom_theme() {
    let (mut app, _shared) = app_with(vec![disk("/", 10.0)]);
    app.prefs.custom_themes.insert("mine".into(), custom(10));
    app.prefs.active_theme = Some("mine".into());

    app.handle_key(key(KeyCode::Char('c')));

    assert!(app.theme_chooser.active);
    let themes = app.all_themes();
    assert_eq!(themes[app.theme_chooser.selected].0, "mine");
}

#[test]
fn chooser_esc_restores_custom_theme_after_previewing_builtin() {
    let (mut app, _shared) = app_with(vec![disk("/", 10.0)]);
    app.prefs.color_mode = ColorMode::Matrix;
    app.prefs.custom_themes.insert("mine".into(), custom(10));
    app.prefs.active_theme = Some("mine".into());

    app.handle_key(key(KeyCode::Char('c')));
    app.handle_key(key(KeyCode::Char('g'))); // first built-in
    assert!(
        app.prefs.active_theme.is_none(),
        "built-in preview clears custom"
    );
    assert_eq!(app.prefs.color_mode, ColorMode::ALL[0]);

    app.handle_key(key(KeyCode::Esc));

    assert!(!app.theme_chooser.active);
    assert_eq!(app.prefs.active_theme.as_deref(), Some("mine"));
    assert_eq!(app.prefs.color_mode, ColorMode::Matrix);
}

#[test]
fn chooser_end_lands_on_last_sorted_custom_theme_and_j_clamps() {
    let (mut app, _shared) = app_with(vec![disk("/", 10.0)]);
    app.prefs.custom_themes.insert("zz".into(), custom(1));
    app.prefs.custom_themes.insert("aa".into(), custom(2));

    app.handle_key(key(KeyCode::Char('c')));
    app.handle_key(key(KeyCode::End));
    let last = ColorMode::ALL.len() + 1;
    assert_eq!(app.theme_chooser.selected, last);
    assert_eq!(app.prefs.active_theme.as_deref(), Some("zz"));

    app.handle_key(key(KeyCode::Char('j')));
    assert_eq!(app.theme_chooser.selected, last);

    app.handle_key(key(KeyCode::Char('k')));
    assert_eq!(app.prefs.active_theme.as_deref(), Some("aa"));
}

#[test]
fn editor_saved_theme_is_selectable_in_chooser() {
    let (mut app, _shared) = app_with(vec![disk("/", 10.0)]);
    app.handle_key(key(KeyCode::Char('C')));
    assert!(app.theme_edit.active);
    app.handle_key(key(KeyCode::Char('L'))); // slot 0 += 10
    let expected_blue = app.theme_edit.colors[0];
    app.handle_key(key(KeyCode::Char('s')));
    for c in "  neon ".chars() {
        app.handle_key(key(KeyCode::Char(c)));
    }
    app.handle_key(key(KeyCode::Enter));

    assert!(!app.theme_edit.active);
    assert_eq!(
        app.prefs.active_theme.as_deref(),
        Some("neon"),
        "name is trimmed"
    );
    assert_eq!(app.prefs.custom_themes["neon"].blue, expected_blue);

    app.handle_key(key(KeyCode::Char('c')));
    assert_eq!(app.all_themes()[app.theme_chooser.selected].0, "neon");
}
