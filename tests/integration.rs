use std::sync::{Arc, Mutex};

use clap::Parser;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use storageshower::app::{mount_col_width, right_col_width, right_col_width_static, App};
use storageshower::cli::Cli;
use storageshower::helpers::{format_bytes, format_uptime, truncate_mount};
use storageshower::prefs::{load_prefs_from, Prefs};
use storageshower::system::{chrono_now, collect_disk_entries, collect_sys_stats, epoch_to_local};
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
        DiskEntry { mount: "/".into(), used: 50_000_000_000, total: 100_000_000_000, pct: 50.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None },
        DiskEntry { mount: "/data".into(), used: 900_000_000_000, total: 1_000_000_000_000, pct: 90.0, kind: DiskKind::HDD, fs: "xfs".into(), latency_ms: None },
        DiskEntry { mount: "/home".into(), used: 80_000_000_000, total: 200_000_000_000, pct: 40.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None },
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

    // Cycle color modes (10 variants)
    for _ in 0..ColorMode::ALL.len() {
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
        latency_ms: None,
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

// ─── CLI → Prefs → App integration ─────────────────────────────────────────

#[test]
fn cli_overrides_applied_to_app() {
    let cli = Cli::parse_from([
        "storageshower", "-s", "size", "-R", "-b", "ascii",
        "-c", "purple", "-u", "gib", "-w", "60", "-C", "85",
        "-r", "5", "--no-bars", "--no-border",
    ]);
    let shared = Arc::new(Mutex::new((SysStats::default(), sample_disks())));
    let app = App::new(Arc::clone(&shared), &cli);
    assert_eq!(app.prefs.sort_mode, SortMode::Size);
    assert!(app.prefs.sort_rev);
    assert_eq!(app.prefs.bar_style, BarStyle::Ascii);
    assert_eq!(app.prefs.color_mode, ColorMode::Purple);
    assert_eq!(app.prefs.unit_mode, UnitMode::GiB);
    assert_eq!(app.prefs.thresh_warn, 60);
    assert_eq!(app.prefs.thresh_crit, 85);
    assert_eq!(app.prefs.refresh_rate, 5);
    assert!(!app.prefs.show_bars);
    assert!(!app.prefs.show_border);
}

#[test]
fn cli_default_preserves_config_defaults() {
    let cli = Cli::parse_from(["storageshower"]);
    let mut prefs = Prefs::default();
    let original = prefs.clone();
    cli.apply_to(&mut prefs);
    assert_eq!(prefs.sort_mode, original.sort_mode);
    assert_eq!(prefs.sort_rev, original.sort_rev);
    assert_eq!(prefs.bar_style, original.bar_style);
    assert_eq!(prefs.color_mode, original.color_mode);
    assert_eq!(prefs.refresh_rate, original.refresh_rate);
}

// ─── Load prefs from nonexistent path ───────────────────────────────────────

#[test]
fn load_prefs_from_nonexistent_returns_defaults() {
    let prefs = load_prefs_from(Some("/tmp/does_not_exist_storageshower.conf"));
    assert_eq!(prefs.sort_mode, SortMode::Name);
    assert_eq!(prefs.refresh_rate, 1);
}

// ─── epoch_to_local known values ────────────────────────────────────────────

#[test]
fn epoch_to_local_unix_epoch() {
    // 2024-01-01 00:00:00 UTC = 1704067200
    // We can't predict local timezone, but we can check the function doesn't panic
    // and returns reasonable values
    let (y, mo, d, h, mi, s) = epoch_to_local(1704067200);
    assert!(y >= 2023 && y <= 2025); // timezone could shift year
    assert!((1..=12).contains(&mo));
    assert!((1..=31).contains(&d));
    assert!(h < 24);
    assert!(mi < 60);
    assert!(s < 60);
}

#[test]
fn epoch_to_local_zero() {
    let (y, mo, d, h, mi, s) = epoch_to_local(0);
    // 1970-01-01 in some timezone
    assert!(y >= 1969 && y <= 1970); // timezone could shift
    assert!((1..=12).contains(&mo));
    assert!((1..=31).contains(&d));
    assert!(h < 24);
    assert!(mi < 60);
    assert!(s < 60);
}

// ─── Format bytes all modes ─────────────────────────────────────────────────

#[test]
fn format_bytes_all_modes_consistency() {
    let size = 1_073_741_824u64; // 1 GiB
    assert_eq!(format_bytes(size, UnitMode::Human), "1.0G");
    assert_eq!(format_bytes(size, UnitMode::GiB), "1.0G");
    assert_eq!(format_bytes(size, UnitMode::MiB), "1024.0M");
    assert_eq!(format_bytes(size, UnitMode::Bytes), "1073741824B");
}

#[test]
fn format_bytes_zero_all_modes() {
    assert_eq!(format_bytes(0, UnitMode::Human), "0B");
    assert_eq!(format_bytes(0, UnitMode::GiB), "0.0G");
    assert_eq!(format_bytes(0, UnitMode::MiB), "0.0M");
    assert_eq!(format_bytes(0, UnitMode::Bytes), "0B");
}

#[test]
fn format_bytes_large_values() {
    let tb = 1_099_511_627_776u64;
    assert_eq!(format_bytes(tb, UnitMode::Human), "1.0T");
    assert_eq!(format_bytes(5 * tb, UnitMode::Human), "5.0T");
}

// ─── Format uptime edge cases ───────────────────────────────────────────────

#[test]
fn format_uptime_zero() {
    assert_eq!(format_uptime(0), "0m");
}

#[test]
fn format_uptime_one_second() {
    assert_eq!(format_uptime(1), "0m");
}

#[test]
fn format_uptime_one_minute() {
    assert_eq!(format_uptime(60), "1m");
}

#[test]
fn format_uptime_one_day_exact() {
    assert_eq!(format_uptime(86400), "1d0h0m");
}

#[test]
fn format_uptime_large() {
    // 365 days
    assert_eq!(format_uptime(365 * 86400), "365d0h0m");
}

// ─── Truncate mount edge cases ──────────────────────────────────────────────

#[test]
fn truncate_mount_width_1() {
    let r = truncate_mount("/very/long", 1);
    assert!(r.chars().count() <= 1);
}

#[test]
fn truncate_mount_width_0() {
    let r = truncate_mount("/hello", 0);
    // Should handle gracefully
    assert!(r.chars().count() <= 1); // ellipsis or empty
}

#[test]
fn truncate_mount_short_string() {
    let r = truncate_mount("/", 20);
    assert_eq!(r.trim_end(), "/");
    assert_eq!(r.chars().count(), 20); // padded
}

// ─── Column widths stress ───────────────────────────────────────────────────

#[test]
fn mount_col_width_all_terminal_sizes() {
    let p = Prefs::default();
    for w in 20..200u16 {
        let col = mount_col_width(w, &p);
        assert!(col >= 12, "mount_col_width({}) = {} < 12", w, col);
        assert!(col <= w as usize, "mount_col_width({}) = {} > {}", w, col, w);
    }
}

#[test]
fn right_col_width_with_different_unit_modes() {
    let disks = vec![
        DiskEntry { mount: "/".into(), used: 50_000_000_000, total: 100_000_000_000, pct: 50.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None },
    ];
    for mode in [UnitMode::Human, UnitMode::GiB, UnitMode::MiB, UnitMode::Bytes] {
        let shared = Arc::new(Mutex::new((SysStats::default(), disks.clone())));
        let mut app = App::new_default(shared);
        app.disks = disks.clone();
        app.prefs = Prefs::default();
        app.prefs.unit_mode = mode;
        let w = right_col_width(&app);
        assert!(w >= 22, "right_col_width with {:?} = {} < 22", mode, w);
    }
}

// ─── Sort stability with equal elements ─────────────────────────────────────

#[test]
fn sort_stability_equal_pct() {
    let disks = vec![
        DiskEntry { mount: "/a".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None },
        DiskEntry { mount: "/b".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None },
        DiskEntry { mount: "/c".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None },
    ];
    let mut app = make_app_with_disks(disks);
    app.prefs.sort_mode = SortMode::Pct;
    let sorted = app.sorted_disks();
    // With equal pct, sort should not crash
    assert_eq!(sorted.len(), 3);
}

#[test]
fn sort_stability_equal_size() {
    let disks = vec![
        DiskEntry { mount: "/a".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None },
        DiskEntry { mount: "/b".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None },
    ];
    let mut app = make_app_with_disks(disks);
    app.prefs.sort_mode = SortMode::Size;
    let sorted = app.sorted_disks();
    assert_eq!(sorted.len(), 2);
}

// ─── Stress: many disks ─────────────────────────────────────────────────────

#[test]
fn sort_and_filter_many_disks() {
    let mut disks = Vec::new();
    for i in 0..500 {
        disks.push(DiskEntry {
            mount: format!("/mount_{:04}", i),
            used: i as u64 * 1_000_000,
            total: 1_000_000_000,
            pct: i as f64 / 5.0,
            kind: DiskKind::SSD,
            fs: "ext4".into(),
            latency_ms: None,
        });
    }
    let mut app = make_app_with_disks(disks);

    // Sort by pct
    app.prefs.sort_mode = SortMode::Pct;
    let sorted = app.sorted_disks();
    assert_eq!(sorted.len(), 500);
    assert!(sorted.windows(2).all(|w| w[0].pct <= w[1].pct));

    // Filter
    app.filter = "mount_00".into();
    let filtered = app.sorted_disks();
    assert!(filtered.len() < 500);
    assert!(filtered.iter().all(|d| d.mount.contains("mount_00")));
}

// ─── Disk entries sanity ────────────────────────────────────────────────────

#[test]
fn collect_disk_entries_used_le_total() {
    let disks = collect_disk_entries();
    for d in &disks {
        assert!(d.used <= d.total,
            "Disk {} has used={} > total={}", d.mount, d.used, d.total);
    }
}

#[test]
fn collect_disk_entries_fs_not_empty() {
    let disks = collect_disk_entries();
    for d in &disks {
        assert!(!d.fs.is_empty(), "Disk {} has empty fs type", d.mount);
    }
}

// ─── System stats fields ────────────────────────────────────────────────────

#[test]
fn sys_stats_kernel_not_empty() {
    let sys = System::new_all();
    let stats = collect_sys_stats(&sys);
    // kernel should be populated on real systems
    assert!(!stats.kernel.is_empty(), "kernel should not be empty");
}

#[test]
fn sys_stats_arch_not_empty() {
    let sys = System::new_all();
    let stats = collect_sys_stats(&sys);
    assert!(!stats.arch.is_empty(), "arch should not be empty");
}

#[test]
fn sys_stats_uptime_nonzero() {
    let sys = System::new_all();
    let stats = collect_sys_stats(&sys);
    assert!(stats.uptime > 0, "uptime should be > 0");
}

#[test]
fn sys_stats_process_count_nonzero() {
    let sys = System::new_all();
    let stats = collect_sys_stats(&sys);
    assert!(stats.process_count > 0, "process_count should be > 0");
}

// ─── Full navigation stress ─────────────────────────────────────────────────

#[test]
fn navigate_full_list_down_then_up() {
    let mut app = make_app_with_disks(sample_disks());
    let count = app.sorted_disks().len();

    // Navigate all the way down
    for _ in 0..count + 5 {
        app.handle_key(make_key(KeyCode::Char('j')));
    }
    assert_eq!(app.selected, Some(count - 1));

    // Navigate all the way up
    for _ in 0..count + 5 {
        app.handle_key(make_key(KeyCode::Char('k')));
    }
    assert_eq!(app.selected, Some(0));
}

// ─── Multi-step workflow ────────────────────────────────────────────────────

#[test]
fn workflow_change_all_settings_then_sort() {
    let mut app = make_app_with_disks(sample_disks());

    // Change every display option
    app.handle_key(make_key(KeyCode::Char('b'))); // bar style
    app.handle_key(make_key(KeyCode::Char('c'))); // color
    app.handle_key(make_key(KeyCode::Char('i'))); // unit mode
    app.handle_key(make_key(KeyCode::Char('v'))); // toggle bars
    app.handle_key(make_key(KeyCode::Char('x'))); // toggle border
    app.handle_key(make_key(KeyCode::Char('g'))); // toggle header
    app.handle_key(make_key(KeyCode::Char('d'))); // toggle used
    app.handle_key(make_key(KeyCode::Char('m'))); // toggle compact
    app.handle_key(make_key(KeyCode::Char('w'))); // toggle full mount
    app.handle_key(make_key(KeyCode::Char('t'))); // warn thresh
    app.handle_key(make_key(KeyCode::Char('T'))); // crit thresh
    app.handle_key(make_key(KeyCode::Char('f'))); // refresh rate

    // Now sort and verify it still works
    app.handle_key(make_key(KeyCode::Char('s'))); // sort by size
    let sorted = app.sorted_disks();
    assert!(sorted.windows(2).all(|w| w[0].total <= w[1].total));

    // Navigate
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.selected, Some(0));
}

// ─── Prefs load from custom path ────────────────────────────────────────────

#[test]
fn load_prefs_from_custom_toml() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.conf");
    std::fs::write(&path, r#"
sort_mode = "Pct"
sort_rev = true
show_local = false
refresh_rate = 5
bar_style = "Ascii"
color_mode = "Purple"
thresh_warn = 60
thresh_crit = 85
show_bars = false
show_border = true
show_header = true
compact = true
show_used = false
full_mount = true
"#).unwrap();
    let prefs = load_prefs_from(Some(path.to_str().unwrap()));
    assert_eq!(prefs.sort_mode, SortMode::Pct);
    assert!(prefs.sort_rev);
    assert_eq!(prefs.refresh_rate, 5);
    assert_eq!(prefs.bar_style, BarStyle::Ascii);
    assert_eq!(prefs.color_mode, ColorMode::Purple);
    assert_eq!(prefs.thresh_warn, 60);
    assert_eq!(prefs.thresh_crit, 85);
    assert!(!prefs.show_bars);
    assert!(prefs.compact);
    assert!(!prefs.show_used);
    assert!(prefs.full_mount);
}

#[test]
fn load_prefs_from_invalid_toml_returns_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bad.conf");
    std::fs::write(&path, "this is not valid toml {{{{").unwrap();
    let prefs = load_prefs_from(Some(path.to_str().unwrap()));
    // Should fall back to defaults
    assert_eq!(prefs.sort_mode, SortMode::Name);
    assert_eq!(prefs.refresh_rate, 1);
}
