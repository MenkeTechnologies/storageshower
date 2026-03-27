use std::sync::{Arc, Mutex};

use clap::Parser;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use storageshower::app::{mount_col_width, right_col_width, right_col_width_static, App};
use storageshower::cli::Cli;
use storageshower::helpers::{format_bytes, format_latency, format_rate, format_uptime, truncate_mount};
use storageshower::system::scan_directory;
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
        DiskEntry { mount: "/".into(), used: 50_000_000_000, total: 100_000_000_000, pct: 50.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        DiskEntry { mount: "/data".into(), used: 900_000_000_000, total: 1_000_000_000_000, pct: 90.0, kind: DiskKind::HDD, fs: "xfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        DiskEntry { mount: "/home".into(), used: 80_000_000_000, total: 200_000_000_000, pct: 40.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
    ]
}

fn make_app_with_disks(disks: Vec<DiskEntry>) -> App {
    let stats = SysStats::default();
    let shared = Arc::new(Mutex::new((stats.clone(), disks.clone())));
    let mut app = App::new_default(shared);
    app.disks = disks;
    app.stats = stats;
    app.prefs = Prefs::default();
    app.test_mode = true;
    app.update_sorted();
    app
}

// ─── End-to-end: sort → filter → navigate ──────────────────────────────────

#[test]
fn full_workflow_sort_filter_navigate() {
    let mut app = make_app_with_disks(sample_disks());

    // Sort by size
    app.handle_key(make_key(KeyCode::Char('s')));
    assert_eq!(app.prefs.sort_mode, SortMode::Size);
    app.update_sorted();
    let disks = app.sorted_disks();
    assert!(disks[0].total <= disks[1].total);

    // Filter to only root
    app.handle_key(make_key(KeyCode::Char('/')));
    assert!(app.filter.active);
    // Type "/"  — but "/" also triggers filter mode, so we need a different char
    app.handle_key(make_key(KeyCode::Char('d')));
    app.handle_key(make_key(KeyCode::Char('a')));
    app.handle_key(make_key(KeyCode::Char('t')));
    app.handle_key(make_key(KeyCode::Char('a')));
    app.handle_key(make_key(KeyCode::Enter));
    assert!(!app.filter.active);
    assert_eq!(app.filter.text, "data");

    app.update_sorted();
    let filtered = app.sorted_disks();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].mount, "/data");

    // Navigate: select first
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.selected, Some(0));

    // Clear filter
    app.handle_key(make_key(KeyCode::Char('0')));
    assert!(app.filter.text.is_empty());
    app.update_sorted();
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

    // 'c' now opens theme chooser popup
    app.handle_key(make_key(KeyCode::Char('c')));
    assert!(app.theme_chooser.active);
    app.handle_key(make_key(KeyCode::Esc)); // close it

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
    assert!(app.filter.active);

    // Type "home"
    for c in "home".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    assert_eq!(app.filter.buf, "home");
    assert_eq!(app.filter.cursor, 4);

    // Live filter should show 1 result
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);

    // Move cursor to start (Ctrl+A)
    app.handle_key(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.cursor, 0);

    // Kill to end (Ctrl+K)
    app.handle_key(KeyEvent {
        code: KeyCode::Char('k'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.buf, "");
    // Live filter: all disks visible again
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 3);

    // Esc cancels — but since we entered with empty filter_prev, it restores ""
    app.handle_key(make_key(KeyCode::Esc));
    assert!(!app.filter.active);
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
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
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
        "--color", "purple", "-u", "gib", "-w", "60", "-C", "85",
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
        DiskEntry { mount: "/".into(), used: 50_000_000_000, total: 100_000_000_000, pct: 50.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
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
        DiskEntry { mount: "/a".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        DiskEntry { mount: "/b".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        DiskEntry { mount: "/c".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
    ];
    let mut app = make_app_with_disks(disks);
    app.prefs.sort_mode = SortMode::Pct;
    app.update_sorted();
    let sorted = app.sorted_disks();
    // With equal pct, sort should not crash
    assert_eq!(sorted.len(), 3);
}

#[test]
fn sort_stability_equal_size() {
    let disks = vec![
        DiskEntry { mount: "/a".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        DiskEntry { mount: "/b".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
    ];
    let mut app = make_app_with_disks(disks);
    app.prefs.sort_mode = SortMode::Size;
    app.update_sorted();
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
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        });
    }
    let mut app = make_app_with_disks(disks);

    // Sort by pct
    app.prefs.sort_mode = SortMode::Pct;
    app.update_sorted();
    let sorted = app.sorted_disks();
    assert_eq!(sorted.len(), 500);
    assert!(sorted.windows(2).all(|w| w[0].pct <= w[1].pct));

    // Filter
    app.filter.text = "mount_00".into();
    app.update_sorted();
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
    app.handle_key(make_key(KeyCode::Char('c'))); // open theme chooser
    app.handle_key(make_key(KeyCode::Esc));        // close it
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
    app.update_sorted();
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

// ─── Drill-down state transitions ─────────────────────────────────────────

#[test]
fn enter_drill_down_and_back() {
    let mut app = make_app_with_disks(sample_disks());
    assert_eq!(app.drill.mode, ViewMode::Disks);

    // Select first disk and press Enter
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.selected, Some(0));
    app.handle_key(make_key(KeyCode::Enter));
    assert_eq!(app.drill.mode, ViewMode::DrillDown);
    assert!(!app.drill.path.is_empty());

    // Esc returns to disk view
    app.handle_key(make_key(KeyCode::Esc));
    assert_eq!(app.drill.mode, ViewMode::Disks);
    assert!(app.drill.path.is_empty());
}

#[test]
fn drill_down_navigation() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/tmp".into()];
    app.drill.entries = vec![
        DirEntry { path: "/tmp/a".into(), name: "a".into(), size: 100, is_dir: true },
        DirEntry { path: "/tmp/b".into(), name: "b".into(), size: 50, is_dir: false },
    ];
    app.drill.selected = 0;

    // j moves down
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.drill.selected, 1);

    // k moves up
    app.handle_key(make_key(KeyCode::Char('k')));
    assert_eq!(app.drill.selected, 0);

    // G jumps to end
    app.handle_key(make_key(KeyCode::Char('G')));
    assert_eq!(app.drill.selected, 1);

    // g jumps to start
    app.handle_key(make_key(KeyCode::Char('g')));
    assert_eq!(app.drill.selected, 0);
}

#[test]
fn drill_down_quit() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.handle_key(make_key(KeyCode::Char('q')));
    assert!(app.quit);
}

// ─── Theme editor state transitions ──────────────────────────────────────

#[test]
fn theme_editor_opens_and_closes() {
    let mut app = make_app_with_disks(sample_disks());
    assert!(!app.theme_edit.active);

    // C opens theme editor
    app.handle_key(make_key(KeyCode::Char('C')));
    assert!(app.theme_edit.active);
    assert_eq!(app.theme_edit.slot, 0);

    // Esc closes it
    app.handle_key(make_key(KeyCode::Esc));
    assert!(!app.theme_edit.active);
}

#[test]
fn theme_editor_navigation() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    assert!(app.theme_edit.active);

    // j/k navigate slots
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.theme_edit.slot, 1);
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.theme_edit.slot, 2);
    app.handle_key(make_key(KeyCode::Char('k')));
    assert_eq!(app.theme_edit.slot, 1);

    // l increments color value
    let before = app.theme_edit.colors[1];
    app.handle_key(make_key(KeyCode::Char('l')));
    assert_eq!(app.theme_edit.colors[1], before.wrapping_add(1));

    // h decrements
    app.handle_key(make_key(KeyCode::Char('h')));
    assert_eq!(app.theme_edit.colors[1], before);
}

#[test]
fn theme_editor_save_flow() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    assert!(app.theme_edit.active);

    // Press s to enter naming mode
    app.handle_key(make_key(KeyCode::Char('s')));
    assert!(app.theme_edit.naming);

    // Type a name
    app.handle_key(make_key(KeyCode::Char('t')));
    app.handle_key(make_key(KeyCode::Char('e')));
    app.handle_key(make_key(KeyCode::Char('s')));
    app.handle_key(make_key(KeyCode::Char('t')));
    assert_eq!(app.theme_edit.name, "test");

    // Enter saves
    app.handle_key(make_key(KeyCode::Enter));
    assert!(!app.theme_edit.active);
    assert!(app.prefs.custom_themes.contains_key("test"));
    assert_eq!(app.prefs.active_theme, Some("test".into()));
}

// ─── Format helpers ──────────────────────────────────────────────────────

#[test]
fn format_rate_consistency() {
    // Rates should be human readable
    assert!(format_rate(0.0).contains("B/s"));
    assert!(format_rate(1024.0).contains("K/s"));
    assert!(format_rate(1_048_576.0).contains("M/s"));
    assert!(format_rate(1_073_741_824.0).contains("G/s"));
}

#[test]
fn format_latency_consistency() {
    assert!(format_latency(0.5).contains("ms"));
    assert!(format_latency(100.0).contains("ms"));
    assert!(format_latency(2000.0).contains("s"));
}

// ─── Directory scanning ──────────────────────────────────────────────────

#[test]
fn scan_directory_returns_sorted_by_size() {
    let entries = scan_directory("/tmp");
    // Should be sorted descending by size
    for w in entries.windows(2) {
        assert!(w[0].size >= w[1].size,
            "{} ({}) should be >= {} ({})", w[0].name, w[0].size, w[1].name, w[1].size);
    }
}

#[test]
fn scan_directory_nonexistent_returns_empty() {
    let entries = scan_directory("/nonexistent_path_xyz_12345");
    assert!(entries.is_empty());
}

// ─── SmartHealth enum ────────────────────────────────────────────────────

#[test]
fn smart_health_equality() {
    assert_eq!(SmartHealth::Verified, SmartHealth::Verified);
    assert_ne!(SmartHealth::Verified, SmartHealth::Failing);
    assert_ne!(SmartHealth::Failing, SmartHealth::Unknown);
}

// ─── Custom themes via prefs ─────────────────────────────────────────────

#[test]
fn custom_theme_roundtrip_toml() {
    let mut prefs = Prefs::default();
    prefs.custom_themes.insert("mytest".into(), ThemeColors {
        blue: 27, green: 48, purple: 135,
        light_purple: 141, royal: 63, dark_purple: 99,
    });
    prefs.active_theme = Some("mytest".into());
    let serialized = toml::to_string_pretty(&prefs).unwrap();
    let deserialized: Prefs = toml::from_str(&serialized).unwrap();
    assert!(deserialized.custom_themes.contains_key("mytest"));
    assert_eq!(deserialized.active_theme, Some("mytest".into()));
    let theme = deserialized.custom_themes.get("mytest").unwrap();
    assert_eq!(theme.blue, 27);
    assert_eq!(theme.green, 48);
}

// ─── Color mode cycling with custom themes ───────────────────────────────

#[test]
fn theme_chooser_shows_custom_themes() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.custom_themes.insert("alpha".into(), ThemeColors {
        blue: 1, green: 2, purple: 3, light_purple: 4, royal: 5, dark_purple: 6,
    });

    let themes = app.all_themes();
    assert_eq!(themes.len(), ColorMode::ALL.len() + 1);
    assert_eq!(themes.last().unwrap().0, "alpha");
}

#[test]
fn theme_chooser_selects_custom_theme() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.custom_themes.insert("alpha".into(), ThemeColors {
        blue: 1, green: 2, purple: 3, light_purple: 4, royal: 5, dark_purple: 6,
    });

    app.handle_key(make_key(KeyCode::Char('c')));
    assert!(app.theme_chooser.active);

    app.handle_key(make_key(KeyCode::Char('G')));
    let themes = app.all_themes();
    assert_eq!(app.theme_chooser.selected, themes.len() - 1);

    app.handle_key(make_key(KeyCode::Enter));
    assert!(!app.theme_chooser.active);
    assert_eq!(app.prefs.active_theme, Some("alpha".into()));
}

// ─── Mouse click selection ───────────────────────────────────────────────

#[test]
fn mouse_click_selects_and_drills_down() {
    let mut app = make_app_with_disks(sample_disks());
    assert!(app.selected.is_none());

    // Click on first disk row (border=true, header=true → first disk at row 5)
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 15, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.selected, Some(0));
    assert_eq!(app.drill.mode, ViewMode::Disks);

    // Click same row again → drill down
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 15, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.drill.mode, ViewMode::DrillDown);
}

#[test]
fn mouse_click_different_row_changes_selection() {
    let mut app = make_app_with_disks(sample_disks());

    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 15, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.selected, Some(0));

    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 15, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.selected, Some(1));
    assert_eq!(app.drill.mode, ViewMode::Disks); // did not drill down
}

// ─── Disk free space alerts ──────────────────────────────────────────────

#[test]
fn alert_triggers_on_threshold_crossing() {
    let disks = vec![
        DiskEntry { mount: "/".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
    ];
    let shared = Arc::new(Mutex::new((SysStats::default(), disks)));
    let mut app = App::new_default(shared.clone());
    app.prefs = Prefs::default();
    app.prefs.thresh_warn = 70;
    app.alert.mounts.clear();

    // First refresh: disk at 50% — no alert
    app.refresh_data();
    assert!(app.alert.flash.is_none());

    // Push disk above warning threshold
    {
        let mut lock = shared.lock().unwrap();
        lock.1 = vec![
            DiskEntry { mount: "/".into(), used: 80, total: 100, pct: 80.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        ];
    }
    app.refresh_data();
    assert!(app.alert.flash.is_some());
    assert!(app.alert.mounts.contains("/"));
    assert!(app.status_msg.is_some());
    let msg = &app.status_msg.as_ref().unwrap().0;
    assert!(msg.contains("ALERT"), "Expected alert message, got: {}", msg);
}

#[test]
fn alert_does_not_re_trigger_for_same_mount() {
    let disks = vec![
        DiskEntry { mount: "/".into(), used: 80, total: 100, pct: 80.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
    ];
    let shared = Arc::new(Mutex::new((SysStats::default(), disks)));
    let mut app = App::new_default(shared.clone());
    app.prefs = Prefs::default();
    app.prefs.thresh_warn = 70;

    // First refresh triggers alert
    app.refresh_data();
    assert!(app.alert.flash.is_some());

    // Clear flash, refresh again — should NOT re-trigger
    app.alert.flash = None;
    app.status_msg = None;
    app.refresh_data();
    assert!(app.alert.flash.is_none(), "Alert should not re-trigger for same mount");
}

#[test]
fn alert_clears_when_disk_drops_below_threshold() {
    let disks = vec![
        DiskEntry { mount: "/".into(), used: 80, total: 100, pct: 80.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
    ];
    let shared = Arc::new(Mutex::new((SysStats::default(), disks)));
    let mut app = App::new_default(shared.clone());
    app.prefs = Prefs::default();
    app.prefs.thresh_warn = 70;

    // Trigger alert
    app.refresh_data();
    assert!(app.alert.mounts.contains("/"));

    // Drop below threshold
    {
        let mut lock = shared.lock().unwrap();
        lock.1 = vec![
            DiskEntry { mount: "/".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        ];
    }
    app.refresh_data();
    assert!(!app.alert.mounts.contains("/"), "Mount should be cleared from alert set");
}

// ─── Bookmarks ───────────────────────────────────────────────────────────

#[test]
fn bookmark_persists_in_prefs_roundtrip() {
    let mut prefs = Prefs::default();
    prefs.bookmarks = vec!["/".into(), "/home".into()];
    let serialized = toml::to_string_pretty(&prefs).unwrap();
    let deserialized: Prefs = toml::from_str(&serialized).unwrap();
    assert_eq!(deserialized.bookmarks, vec!["/", "/home"]);
}

#[test]
fn bookmarked_disks_appear_first() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/home".into()];
    app.update_sorted();
    let disks = app.sorted_disks();
    assert_eq!(disks[0].mount, "/home", "Bookmarked disk should be first");
}

#[test]
fn multiple_bookmarks_all_pinned() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/home".into(), "/data".into()];
    app.update_sorted();
    let disks = app.sorted_disks();
    // Both bookmarked disks should be in top 2
    let top2: Vec<&str> = disks.iter().take(2).map(|d| d.mount.as_str()).collect();
    assert!(top2.contains(&"/home"));
    assert!(top2.contains(&"/data"));
}

// ─── Open in file manager (o key) ───────────────────────────────────────

#[test]
fn o_key_does_not_drill_down() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = Some(0);
    app.handle_key(make_key(KeyCode::Char('o')));
    // Should NOT change view mode — just spawns open command
    assert_eq!(app.drill.mode, ViewMode::Disks);
}

// ─── Drill-down backspace navigation ─────────────────────────────────────

#[test]
fn drill_down_backspace_returns_to_disks() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.handle_key(make_key(KeyCode::Backspace));
    assert_eq!(app.drill.mode, ViewMode::Disks);
    assert!(app.drill.path.is_empty());
}

#[test]
fn drill_down_backspace_goes_up_one_level() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into(), "/usr".into()];
    app.handle_key(make_key(KeyCode::Backspace));
    assert_eq!(app.drill.mode, ViewMode::DrillDown);
    assert_eq!(app.drill.path, vec!["/"]);
}

// ─── SmartHealth display values ──────────────────────────────────────────

#[test]
fn smart_health_copy_and_debug() {
    let v = SmartHealth::Verified;
    let c = v;
    assert_eq!(c, SmartHealth::Verified);
    assert_eq!(format!("{:?}", SmartHealth::Failing), "Failing");
    assert_eq!(format!("{:?}", SmartHealth::Unknown), "Unknown");
}

// ─── DirEntry fields ────────────────────────────────────────────────────

#[test]
fn dir_entry_clone_and_fields() {
    let e = DirEntry {
        path: "/tmp/test".into(),
        name: "test".into(),
        size: 1024,
        is_dir: true,
    };
    let c = e.clone();
    assert_eq!(c.path, "/tmp/test");
    assert_eq!(c.name, "test");
    assert_eq!(c.size, 1024);
    assert!(c.is_dir);
}

// ─── ViewMode enum ──────────────────────────────────────────────────────

#[test]
fn view_mode_equality() {
    assert_eq!(ViewMode::Disks, ViewMode::Disks);
    assert_eq!(ViewMode::DrillDown, ViewMode::DrillDown);
    assert_ne!(ViewMode::Disks, ViewMode::DrillDown);
}

// ─── Drill-down sort modes ──────────────────────────────────────────────

#[test]
fn drill_sort_by_name() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.drill.entries = vec![
        DirEntry { path: "/c".into(), name: "charlie".into(), size: 10, is_dir: false },
        DirEntry { path: "/a".into(), name: "alpha".into(), size: 30, is_dir: true },
        DirEntry { path: "/b".into(), name: "bravo".into(), size: 20, is_dir: false },
    ];
    app.handle_key(make_key(KeyCode::Char('n')));
    assert_eq!(app.drill.sort, DrillSortMode::Name);
    assert_eq!(app.drill.entries[0].name, "alpha");
    assert_eq!(app.drill.entries[1].name, "bravo");
    assert_eq!(app.drill.entries[2].name, "charlie");
}

#[test]
fn drill_sort_by_size() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 10, is_dir: false },
        DirEntry { path: "/b".into(), name: "b".into(), size: 30, is_dir: false },
        DirEntry { path: "/c".into(), name: "c".into(), size: 20, is_dir: false },
    ];
    // Default is size desc, switch to name then back to size
    app.handle_key(make_key(KeyCode::Char('n')));
    app.handle_key(make_key(KeyCode::Char('s')));
    assert_eq!(app.drill.sort, DrillSortMode::Size);
    assert_eq!(app.drill.entries[0].size, 30); // largest first
    assert_eq!(app.drill.entries[2].size, 10);
}

#[test]
fn drill_sort_reverse() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 30, is_dir: false },
        DirEntry { path: "/b".into(), name: "b".into(), size: 10, is_dir: false },
    ];
    // Default size desc: 30, 10
    app.sort_drill_entries();
    assert_eq!(app.drill.entries[0].size, 30);

    // Reverse
    app.handle_key(make_key(KeyCode::Char('r')));
    assert!(app.drill.sort_rev);
    assert_eq!(app.drill.entries[0].size, 10); // smallest first now
}

#[test]
fn drill_sort_toggle_same_mode_reverses() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 10, is_dir: false },
    ];
    assert_eq!(app.drill.sort, DrillSortMode::Size);
    assert!(!app.drill.sort_rev);

    // Press s again — should toggle reverse
    app.handle_key(make_key(KeyCode::Char('s')));
    assert!(app.drill.sort_rev);
}

#[test]
fn drill_scan_progress_counters() {
    let mut app = make_app_with_disks(sample_disks());
    // Before scan, counters are zero
    assert_eq!(*app.drill.scan_count.lock().unwrap(), 0);
    assert_eq!(*app.drill.scan_total.lock().unwrap(), 0);

    // Start a scan of /tmp
    app.selected = Some(0);
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/tmp".into()];
    // Simulate by directly calling start_drill_scan
    app.start_drill_scan("/tmp");
    assert!(app.drill.scanning);

    // Wait for scan to complete
    std::thread::sleep(std::time::Duration::from_millis(500));
    app.refresh_data();

    // After completion, scanning should be false
    assert!(!app.drill.scanning);
    // Total should have been set (may be 0 if /tmp is empty)
    let total = *app.drill.scan_total.lock().unwrap();
    let count = *app.drill.scan_count.lock().unwrap();
    assert_eq!(count, total, "count should equal total after completion");
}

#[test]
fn drill_sort_resets_selection() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 10, is_dir: false },
        DirEntry { path: "/b".into(), name: "b".into(), size: 20, is_dir: false },
    ];
    app.drill.selected = 1;
    app.handle_key(make_key(KeyCode::Char('n')));
    assert_eq!(app.drill.selected, 0, "Sort should reset selection to 0");
}

// ─── Export theme CLI ────────────────────────────────────────────────────

#[test]
fn hover_sets_position() {
    let mut app = make_app_with_disks(sample_disks());
    assert!(app.hover.pos.is_none());
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 20, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.hover.pos, Some((20, 6)));
}

#[test]
fn hover_resolves_disk_index() {
    let mut app = make_app_with_disks(sample_disks());
    // With border + header, first disk row is at y=5
    app.hover.pos = Some((10, 5));
    assert_eq!(app.hovered_disk_index(), Some(0));
    app.hover.pos = Some((10, 6));
    assert_eq!(app.hovered_disk_index(), Some(1));
}

#[test]
fn hover_out_of_range_returns_none() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 50));
    assert!(app.hovered_disk_index().is_none());
    app.hover.pos = Some((10, 0)); // title bar
    assert!(app.hovered_disk_index().is_none());
}

#[test]
fn hover_none_returns_none() {
    let app = make_app_with_disks(sample_disks());
    assert!(app.hover.pos.is_none());
    assert!(app.hovered_disk_index().is_none());
}

#[test]
fn export_theme_flag_parses() {
    let cli = Cli::parse_from(["storageshower", "--export-theme"]);
    assert!(cli.export_theme);
}

#[test]
fn export_theme_with_color_flag() {
    let cli = Cli::parse_from(["storageshower", "--export-theme", "--color", "purple"]);
    assert!(cli.export_theme);
    assert_eq!(cli.color_mode, Some(ColorMode::Purple));
}

#[test]
fn export_theme_with_theme_flag() {
    let cli = Cli::parse_from(["storageshower", "--export-theme", "--theme", "mytest"]);
    assert!(cli.export_theme);
    assert_eq!(cli.theme, Some("mytest".into()));
}

// ─── HoverZone detection ─────────────────────────────────────────────────

#[test]
fn hover_zone_title_bar() {
    let mut app = make_app_with_disks(sample_disks());
    // Title bar is at row 1 with border
    app.hover.pos = Some((10, 1));
    assert_eq!(app.hovered_zone(40), HoverZone::TitleBar);
}

#[test]
fn hover_zone_footer_bar() {
    let mut app = make_app_with_disks(sample_disks());
    // Footer is near the bottom: h - footer_rows + 1
    // With border, footer_rows = 3, so footer_row = 40 - 3 + 1 = 38
    app.hover.pos = Some((10, 38));
    assert_eq!(app.hovered_zone(40), HoverZone::FooterBar);
}

#[test]
fn hover_zone_disk_row() {
    let mut app = make_app_with_disks(sample_disks());
    // First disk at row 5 (border + title + sep + header + sep)
    app.hover.pos = Some((10, 5));
    assert_eq!(app.hovered_zone(40), HoverZone::DiskRow(0));
    app.hover.pos = Some((10, 6));
    assert_eq!(app.hovered_zone(40), HoverZone::DiskRow(1));
}

#[test]
fn hover_zone_none_on_separator() {
    let mut app = make_app_with_disks(sample_disks());
    // Row 0 is border, not title
    app.hover.pos = Some((10, 0));
    assert_eq!(app.hovered_zone(40), HoverZone::None);
}

#[test]
fn hover_zone_none_when_no_hover() {
    let app = make_app_with_disks(sample_disks());
    assert_eq!(app.hovered_zone(40), HoverZone::None);
}

// ─── Hover delay ─────────────────────────────────────────────────────────

#[test]
fn hover_not_ready_immediately() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 5));
    app.hover.since = Some(std::time::Instant::now());
    assert!(!app.hover_ready(), "Hover should not be ready immediately");
}

#[test]
fn hover_ready_after_delay() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 5));
    app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_secs(3));
    assert!(app.hover_ready(), "Hover should be ready after 2+ seconds");
}

#[test]
fn hover_resets_on_move() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 5));
    app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_secs(3));
    assert!(app.hover_ready());

    // Move to different position
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 20, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(!app.hover_ready(), "Hover delay should reset on move");
}

// ─── Scroll offset ───────────────────────────────────────────────────────

#[test]
fn scroll_offset_follows_selection_down() {
    let mut app = make_app_with_disks(sample_disks());
    app.scroll_offset = 0;
    app.selected = Some(2);
    app.ensure_visible(2); // only 2 visible rows
    assert_eq!(app.scroll_offset, 1, "Should scroll to keep selection visible");
}

#[test]
fn scroll_offset_follows_selection_up() {
    let mut app = make_app_with_disks(sample_disks());
    app.scroll_offset = 2;
    app.selected = Some(0);
    app.ensure_visible(2);
    assert_eq!(app.scroll_offset, 0, "Should scroll up to show selection");
}

#[test]
fn scroll_offset_stays_when_visible() {
    let mut app = make_app_with_disks(sample_disks());
    app.scroll_offset = 0;
    app.selected = Some(1);
    app.ensure_visible(3);
    assert_eq!(app.scroll_offset, 0, "Should not scroll when selection is visible");
}

#[test]
fn drill_scroll_offset_follows_selection() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.selected = 5;
    app.drill.scroll_offset = 0;
    app.ensure_drill_visible(3);
    assert_eq!(app.drill.scroll_offset, 3);
}

// ─── Mouse scroll wheel ─────────────────────────────────────────────────

#[test]
fn mouse_scroll_down_selects_next() {
    let mut app = make_app_with_disks(sample_disks());
    assert!(app.selected.is_none());
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::ScrollDown, column: 10, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.selected, Some(0));
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::ScrollDown, column: 10, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.selected, Some(1));
}

#[test]
fn mouse_scroll_up_selects_prev() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = Some(2);
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::ScrollUp, column: 10, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.selected, Some(1));
}

// ─── Right-click triggers instant tooltip ────────────────────────────────

#[test]
fn right_click_sets_hover_instantly() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 30, row: 7, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.hover.pos, Some((30, 7)));
    assert!(app.hover_ready());
    assert!(!app.show_help, "Right-click should not toggle help");
}

// ─── Drill-down hover index ─────────────────────────────────────────────

#[test]
fn hovered_drill_index_resolves() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 10, is_dir: true },
        DirEntry { path: "/b".into(), name: "b".into(), size: 20, is_dir: false },
    ];
    // First entry at row 5 (border=1 + 4 chrome rows)
    app.hover.pos = Some((10, 5));
    assert_eq!(app.hovered_drill_index(), Some(0));
    app.hover.pos = Some((10, 6));
    assert_eq!(app.hovered_drill_index(), Some(1));
    app.hover.pos = Some((10, 50));
    assert!(app.hovered_drill_index().is_none());
}

// ─── Hover delay value ──────────────────────────────────────────────────

#[test]
fn hover_delay_is_one_second() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 5));
    // Just under 1 second — not ready
    app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_millis(900));
    assert!(!app.hover_ready());
    // Over 1 second — ready
    app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_millis(1100));
    assert!(app.hover_ready());
}

// ─── HoverZone consistency ───────────────────────────────────────────────

#[test]
fn hover_zone_matches_hovered_disk_index() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 5));
    assert_eq!(app.hovered_zone(40), HoverZone::DiskRow(0));
    assert_eq!(app.hovered_disk_index(), Some(0));

    app.hover.pos = Some((10, 7));
    assert_eq!(app.hovered_zone(40), HoverZone::DiskRow(2));
    assert_eq!(app.hovered_disk_index(), Some(2));
}

#[test]
fn hover_zone_title_not_disk() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 1));
    assert_eq!(app.hovered_zone(40), HoverZone::TitleBar);
    assert!(app.hovered_disk_index().is_none());
}

// ─── Drill-down hover index with scroll offset ──────────────────────────

#[test]
fn drill_hover_index_none_on_header() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 10, is_dir: true },
    ];
    // Row 3 is header area in drill-down (border + breadcrumb + sep + header)
    app.hover.pos = Some((10, 3));
    assert!(app.hovered_drill_index().is_none());
}

// ─── Version string accessible ──────────────────────────────────────────

#[test]
fn cargo_version_available() {
    let ver = env!("CARGO_PKG_VERSION");
    assert!(!ver.is_empty());
    assert!(ver.contains('.'));
}

// ─── Theme chooser popup ────────────────────────────────────────────────

#[test]
fn theme_chooser_opens_with_c() {
    let mut app = make_app_with_disks(sample_disks());
    assert!(!app.theme_chooser.active);
    app.handle_key(make_key(KeyCode::Char('c')));
    assert!(app.theme_chooser.active);
}

#[test]
fn theme_chooser_preselects_current() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.color_mode = ColorMode::Blue;
    app.handle_key(make_key(KeyCode::Char('c')));
    // Blue is index 2 in ColorMode::ALL
    let expected = app.all_themes().iter().position(|(k, _)| k == "blue").unwrap();
    assert_eq!(app.theme_chooser.selected, expected);
}

#[test]
fn theme_chooser_navigate_bounds() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('c')));

    // Navigate past end
    for _ in 0..100 {
        app.handle_key(make_key(KeyCode::Char('j')));
    }
    let count = app.all_themes().len();
    assert_eq!(app.theme_chooser.selected, count - 1);

    // Navigate past start
    for _ in 0..100 {
        app.handle_key(make_key(KeyCode::Char('k')));
    }
    assert_eq!(app.theme_chooser.selected, 0);
}

#[test]
fn theme_chooser_g_jumps() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('c')));

    app.handle_key(make_key(KeyCode::Char('G')));
    assert_eq!(app.theme_chooser.selected, app.all_themes().len() - 1);

    app.handle_key(make_key(KeyCode::Char('g')));
    assert_eq!(app.theme_chooser.selected, 0);
}

#[test]
fn theme_chooser_enter_applies_builtin() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('c')));
    // Select Purple (index 3)
    app.theme_chooser.selected = 3;
    app.handle_key(make_key(KeyCode::Enter));
    assert!(!app.theme_chooser.active);
    assert_eq!(app.prefs.color_mode, ColorMode::Purple);
    assert!(app.prefs.active_theme.is_none());
}

#[test]
fn all_themes_includes_builtins_and_custom() {
    let mut app = make_app_with_disks(sample_disks());
    assert_eq!(app.all_themes().len(), ColorMode::ALL.len());
    app.prefs.custom_themes.insert("zeta".into(), ThemeColors {
        blue: 1, green: 2, purple: 3, light_purple: 4, royal: 5, dark_purple: 6,
    });
    app.prefs.custom_themes.insert("alpha".into(), ThemeColors {
        blue: 1, green: 2, purple: 3, light_purple: 4, royal: 5, dark_purple: 6,
    });
    let themes = app.all_themes();
    assert_eq!(themes.len(), ColorMode::ALL.len() + 2);
    // Custom themes sorted alphabetically after builtins
    assert_eq!(themes[ColorMode::ALL.len()].0, "alpha");
    assert_eq!(themes[ColorMode::ALL.len() + 1].0, "zeta");
}

// ─── Theme chooser live preview ──────────────────────────────────────────

#[test]
fn theme_chooser_live_preview_on_navigate() {
    let mut app = make_app_with_disks(sample_disks());
    assert_eq!(app.prefs.color_mode, ColorMode::Default);
    app.handle_key(make_key(KeyCode::Char('c')));
    // Navigate down should auto-apply
    app.handle_key(make_key(KeyCode::Down));
    assert_eq!(app.prefs.color_mode, ColorMode::ALL[1]);
    app.handle_key(make_key(KeyCode::Down));
    assert_eq!(app.prefs.color_mode, ColorMode::ALL[2]);
    // Up arrow goes back
    app.handle_key(make_key(KeyCode::Up));
    assert_eq!(app.prefs.color_mode, ColorMode::ALL[1]);
}

#[test]
fn theme_chooser_esc_reverts_after_preview() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.color_mode = ColorMode::Blue;
    app.handle_key(make_key(KeyCode::Char('c')));
    // Navigate around
    for _ in 0..5 {
        app.handle_key(make_key(KeyCode::Char('j')));
    }
    assert_ne!(app.prefs.color_mode, ColorMode::Blue);
    // Esc reverts
    app.handle_key(make_key(KeyCode::Esc));
    assert!(!app.theme_chooser.active);
    assert_eq!(app.prefs.color_mode, ColorMode::Blue);
}

#[test]
fn theme_chooser_enter_confirms_preview() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('c')));
    // Navigate to index 4
    for _ in 0..4 {
        app.handle_key(make_key(KeyCode::Char('j')));
    }
    let expected = ColorMode::ALL[4];
    assert_eq!(app.prefs.color_mode, expected);
    app.handle_key(make_key(KeyCode::Enter));
    assert!(!app.theme_chooser.active);
    // Theme stays applied after confirm
    assert_eq!(app.prefs.color_mode, expected);
    assert!(app.status_msg.is_some());
}

#[test]
fn theme_chooser_enter_applies_custom_theme() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.custom_themes.insert("mytest".into(), ThemeColors {
        blue: 100, green: 101, purple: 102, light_purple: 103, royal: 104, dark_purple: 105,
    });
    app.handle_key(make_key(KeyCode::Char('c')));
    // Custom themes appear after builtins
    let custom_idx = ColorMode::ALL.len();
    app.theme_chooser.selected = custom_idx;
    app.handle_key(make_key(KeyCode::Enter));
    assert_eq!(app.prefs.active_theme, Some("mytest".into()));
}

// ─── Theme chooser mouse interaction ─────────────────────────────────────

#[test]
fn theme_chooser_mouse_click_applies_theme() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('c')));
    let themes = app.all_themes();
    let box_h = (themes.len() as u16 + 4).min(20u16); // 24 - 4 = 20
    let y0 = (24u16.saturating_sub(box_h)) / 2;
    // Click third row in content area
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 30, row: y0 + 2 + 2, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.theme_chooser.selected, 2);
    assert_eq!(app.prefs.color_mode, ColorMode::ALL[2]);
    assert!(app.theme_chooser.active); // Still open
}

#[test]
fn theme_chooser_click_outside_reverts() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.color_mode = ColorMode::Purple;
    app.handle_key(make_key(KeyCode::Char('c')));
    // Navigate to change theme
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_ne!(app.prefs.color_mode, ColorMode::Purple);
    // Click outside
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 0, row: 0, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(!app.theme_chooser.active);
    assert_eq!(app.prefs.color_mode, ColorMode::Purple);
}

#[test]
fn theme_chooser_scroll_auto_applies() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('c')));
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::ScrollDown, column: 40, row: 12, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.theme_chooser.selected, 1);
    assert_eq!(app.prefs.color_mode, ColorMode::ALL[1]);
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::ScrollDown, column: 40, row: 12, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.theme_chooser.selected, 2);
    assert_eq!(app.prefs.color_mode, ColorMode::ALL[2]);
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::ScrollUp, column: 40, row: 12, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.theme_chooser.selected, 1);
    assert_eq!(app.prefs.color_mode, ColorMode::ALL[1]);
}

// ─── Right-click tooltip flag ──────────────────────────────────────────────

#[test]
fn right_click_sets_right_click_flag() {
    let mut app = make_app_with_disks(sample_disks());
    assert!(!app.hover.right_click);
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 10, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.right_click);
    assert!(app.hover_ready());
}

#[test]
fn hover_move_clears_right_click_flag() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 10, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.right_click);
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 20, row: 8, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(!app.hover.right_click);
}

#[test]
fn hover_move_same_pos_preserves_right_click() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 10, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.right_click);
    // Same position — flag preserved
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 10, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.right_click);
}

// ─── Drag pct/bar-end separators ───────────────────────────────────────────

#[test]
fn drag_pct_separator_resizes_column() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.show_used = true;
    let right_w = right_col_width(&app);
    let pct_w: u16 = if app.prefs.col_pct_w > 0 { app.prefs.col_pct_w } else { 5 };
    let rm: u16 = 1; // show_border default
    let right_start = 80u16.saturating_sub(rm + right_w);
    let pct_sep_x = right_start + pct_w;

    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: pct_sep_x, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(matches!(app.drag, Some(DragTarget::PctSep)));

    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Drag(MouseButton::Left), column: pct_sep_x + 2, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.prefs.col_pct_w > 0);

    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), column: pct_sep_x + 2, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.drag.is_none());
}

#[test]
fn drag_bar_end_separator_resizes_column() {
    let mut app = make_app_with_disks(sample_disks());
    let right_w = right_col_width(&app);
    let rm: u16 = 1;
    let bar_end_x = 80u16.saturating_sub(rm + right_w + 1);

    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: bar_end_x, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(matches!(app.drag, Some(DragTarget::BarEndSep)));

    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Drag(MouseButton::Left), column: bar_end_x - 4, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.prefs.col_bar_end_w > 0);

    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), column: bar_end_x - 4, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.drag.is_none());
}

#[test]
fn drag_on_header_row_takes_priority_over_sort() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.show_used = true;
    let right_w = right_col_width(&app);
    let pct_w: u16 = if app.prefs.col_pct_w > 0 { app.prefs.col_pct_w } else { 5 };
    let rm: u16 = 1;
    let right_start = 80u16.saturating_sub(rm + right_w);
    let pct_sep_x = right_start + pct_w;
    let header_row: u16 = 3; // border + title + sep

    let sort_before = app.prefs.sort_mode;
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: pct_sep_x, row: header_row, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    // Should start drag, NOT change sort
    assert!(matches!(app.drag, Some(DragTarget::PctSep)));
    assert_eq!(app.prefs.sort_mode, sort_before);
}

// ─── Hover early cancellation on mouse move ──────────────────────────────

#[test]
fn hover_cancelled_on_move_before_any_handler() {
    let mut app = make_app_with_disks(sample_disks());

    // Start hover timer on title bar
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 10, row: 1, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.since.is_some());

    // Move to empty zone — timer should be cancelled
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 10, row: 0, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.since.is_none());
}

#[test]
fn hover_cancelled_during_theme_chooser() {
    let mut app = make_app_with_disks(sample_disks());

    // Start hover on disk row
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 10, row: 5, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.since.is_some());

    // Open theme chooser
    app.handle_key(make_key(KeyCode::Char('c')));
    assert!(app.theme_chooser.active);

    // Mouse move inside theme chooser — hover timer must be cancelled
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 30, row: 12, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.since.is_none());
    assert!(!app.hover.right_click);
}

#[test]
fn hover_restarted_on_valid_zone_after_cancel() {
    let mut app = make_app_with_disks(sample_disks());

    // Move to empty zone — no timer
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 10, row: 0, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.since.is_none());

    // Move to title bar — timer should restart
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 10, row: 1, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.since.is_some());
}

// ─── Hover auto-hide ─────────────────────────────────────────────────────

#[test]
fn hover_auto_hides_after_display_window() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 1));

    // Set hover start to 5s ago — past 1s delay and 4s auto-hide window
    app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_secs(5));
    app.hover.right_click = false;
    assert!(!app.hover_ready(), "auto-hover should hide after 4s");
}

#[test]
fn hover_visible_during_display_window() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 1));

    // Set hover start to 2s ago — past 1s delay, within 4s auto-hide
    app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_millis(2000));
    app.hover.right_click = false;
    assert!(app.hover_ready(), "auto-hover should be visible between 1-4s");
}

#[test]
fn right_click_hover_persists_indefinitely() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((10, 5));

    // Set right-click hover start to 60s ago
    app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_secs(60));
    app.hover.right_click = true;
    assert!(app.hover_ready(), "right-click hover should not auto-hide");
}

#[test]
fn hover_workflow_move_wait_autohide() {
    let mut app = make_app_with_disks(sample_disks());

    // Move to title bar
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Moved, column: 10, row: 1, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.hover.since.is_some());
    assert!(!app.hover_ready(), "should not be ready immediately");

    // Simulate time passing — replace since with 2s ago (visible window)
    app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_millis(2000));
    assert!(app.hover_ready(), "should be visible after 1s delay");

    // Simulate more time — replace since with 5s ago (past auto-hide)
    app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_secs(5));
    assert!(!app.hover_ready(), "should auto-hide after 4s");
}

// ═══════════════════════════════════════════════════════════════════════════
// NEW INTEGRATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

// ─── Filter emacs-style editing: Ctrl+U, Ctrl+W, Ctrl+H ─────────────────

#[test]
fn filter_ctrl_u_kills_to_start() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "hello".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    assert_eq!(app.filter.buf, "hello");
    // Ctrl+U kills from cursor to start
    app.handle_key(KeyEvent {
        code: KeyCode::Char('u'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.buf, "");
    assert_eq!(app.filter.cursor, 0);
}

#[test]
fn filter_ctrl_w_kills_word() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "foo bar".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    assert_eq!(app.filter.buf, "foo bar");
    // Ctrl+W kills previous word
    app.handle_key(KeyEvent {
        code: KeyCode::Char('w'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.buf, "foo ");
}

#[test]
fn filter_ctrl_h_deletes_backward() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "abc".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    app.handle_key(KeyEvent {
        code: KeyCode::Char('h'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.buf, "ab");
}

#[test]
fn filter_ctrl_b_moves_left() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "abc".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    assert_eq!(app.filter.cursor, 3);
    app.handle_key(KeyEvent {
        code: KeyCode::Char('b'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.cursor, 2);
}

#[test]
fn filter_ctrl_f_moves_right() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "abc".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    // Move to start, then Ctrl+F forward
    app.handle_key(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.cursor, 0);
    app.handle_key(KeyEvent {
        code: KeyCode::Char('f'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.cursor, 1);
}

#[test]
fn filter_ctrl_e_moves_to_end() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "test".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    app.handle_key(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.cursor, 0);
    app.handle_key(KeyEvent {
        code: KeyCode::Char('e'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.filter.cursor, 4);
}

#[test]
fn filter_home_end_keys() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "test".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    app.handle_key(make_key(KeyCode::Home));
    assert_eq!(app.filter.cursor, 0);
    app.handle_key(make_key(KeyCode::End));
    assert_eq!(app.filter.cursor, 4);
}

#[test]
fn filter_left_right_arrow_keys() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "ab".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    app.handle_key(make_key(KeyCode::Left));
    assert_eq!(app.filter.cursor, 1);
    app.handle_key(make_key(KeyCode::Right));
    assert_eq!(app.filter.cursor, 2);
}

#[test]
fn filter_delete_key() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "abc".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    // Move to position 1, delete 'b'
    app.handle_key(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    app.handle_key(make_key(KeyCode::Right));
    app.handle_key(make_key(KeyCode::Delete));
    assert_eq!(app.filter.buf, "ac");
}

#[test]
fn filter_backspace_at_start_does_nothing() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "x".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    app.handle_key(make_key(KeyCode::Home));
    app.handle_key(make_key(KeyCode::Backspace));
    assert_eq!(app.filter.buf, "x");
    assert_eq!(app.filter.cursor, 0);
}

#[test]
fn filter_esc_restores_previous() {
    let mut app = make_app_with_disks(sample_disks());
    // Set existing filter
    app.filter.text = "old".into();
    // Enter filter mode
    app.handle_key(make_key(KeyCode::Char('/')));
    assert_eq!(app.filter.prev, "old");
    // Type new text
    for c in "new".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    // Esc restores
    app.handle_key(make_key(KeyCode::Esc));
    assert_eq!(app.filter.text, "old");
    assert!(!app.filter.active);
}

#[test]
fn filter_enter_confirms() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "new".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    app.handle_key(make_key(KeyCode::Enter));
    assert_eq!(app.filter.text, "new");
    assert!(!app.filter.active);
}

#[test]
fn filter_insert_at_cursor_position() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "ac".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    // Move left once, insert 'b'
    app.handle_key(make_key(KeyCode::Left));
    app.handle_key(make_key(KeyCode::Char('b')));
    assert_eq!(app.filter.buf, "abc");
    assert_eq!(app.filter.cursor, 2);
}

// ─── Navigation: Ctrl+D / Ctrl+U half-page ──────────────────────────────

#[test]
fn ctrl_d_half_page_down() {
    let mut disks = Vec::new();
    for i in 0..20 {
        disks.push(DiskEntry {
            mount: format!("/mnt/{}", i), used: 50, total: 100, pct: 50.0,
            kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
    }
    let mut app = make_app_with_disks(disks);
    app.selected = Some(0);
    app.handle_key(KeyEvent {
        code: KeyCode::Char('d'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.selected, Some(10)); // half of 20
}

#[test]
fn ctrl_u_half_page_up() {
    let mut disks = Vec::new();
    for i in 0..20 {
        disks.push(DiskEntry {
            mount: format!("/mnt/{}", i), used: 50, total: 100, pct: 50.0,
            kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
    }
    let mut app = make_app_with_disks(disks);
    app.selected = Some(15);
    app.handle_key(KeyEvent {
        code: KeyCode::Char('u'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.selected, Some(5)); // 15 - 10
}

#[test]
fn ctrl_d_from_none_starts_at_half() {
    let mut disks = Vec::new();
    for i in 0..10 {
        disks.push(DiskEntry {
            mount: format!("/mnt/{}", i), used: 50, total: 100, pct: 50.0,
            kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
    }
    let mut app = make_app_with_disks(disks);
    assert!(app.selected.is_none());
    app.handle_key(KeyEvent {
        code: KeyCode::Char('d'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.selected, Some(5));
}

#[test]
fn ctrl_u_from_none_goes_to_zero() {
    let mut disks = Vec::new();
    for i in 0..10 {
        disks.push(DiskEntry {
            mount: format!("/mnt/{}", i), used: 50, total: 100, pct: 50.0,
            kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
    }
    let mut app = make_app_with_disks(disks);
    assert!(app.selected.is_none());
    app.handle_key(KeyEvent {
        code: KeyCode::Char('u'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.selected, Some(0));
}

#[test]
fn ctrl_g_selects_first() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = Some(2);
    app.handle_key(KeyEvent {
        code: KeyCode::Char('g'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert_eq!(app.selected, Some(0));
}

// ─── Navigation: Home/End/G ─────────────────────────────────────────────

#[test]
fn home_selects_first() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = Some(2);
    app.handle_key(make_key(KeyCode::Home));
    assert_eq!(app.selected, Some(0));
}

#[test]
fn end_selects_last() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = Some(0);
    app.handle_key(make_key(KeyCode::End));
    assert_eq!(app.selected, Some(app.sorted_disks().len() - 1));
}

#[test]
fn big_g_selects_last() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = Some(0);
    app.handle_key(make_key(KeyCode::Char('G')));
    assert_eq!(app.selected, Some(app.sorted_disks().len() - 1));
}

// ─── Key toggles ────────────────────────────────────────────────────────

#[test]
fn p_toggles_pause() {
    let mut app = make_app_with_disks(sample_disks());
    assert!(!app.paused);
    app.handle_key(make_key(KeyCode::Char('p')));
    assert!(app.paused);
    app.handle_key(make_key(KeyCode::Char('p')));
    assert!(!app.paused);
}

#[test]
fn l_toggles_local_only() {
    let mut app = make_app_with_disks(sample_disks());
    let before = app.prefs.show_local;
    app.handle_key(make_key(KeyCode::Char('l')));
    assert_eq!(app.prefs.show_local, !before);
    app.handle_key(make_key(KeyCode::Char('l')));
    assert_eq!(app.prefs.show_local, before);
}

#[test]
fn a_toggles_show_all() {
    let mut app = make_app_with_disks(sample_disks());
    let before = app.prefs.show_all;
    app.handle_key(make_key(KeyCode::Char('a')));
    assert_eq!(app.prefs.show_all, !before);
}

#[test]
fn v_toggles_bars() {
    let mut app = make_app_with_disks(sample_disks());
    let before = app.prefs.show_bars;
    app.handle_key(make_key(KeyCode::Char('v')));
    assert_eq!(app.prefs.show_bars, !before);
}

#[test]
fn d_toggles_used_and_resets_bar_end() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.col_bar_end_w = 30;
    let before = app.prefs.show_used;
    app.handle_key(make_key(KeyCode::Char('d')));
    assert_eq!(app.prefs.show_used, !before);
    assert_eq!(app.prefs.col_bar_end_w, 0, "col_bar_end_w should reset on toggle");
}

#[test]
fn g_toggles_header() {
    let mut app = make_app_with_disks(sample_disks());
    let before = app.prefs.show_header;
    app.handle_key(make_key(KeyCode::Char('g')));
    assert_eq!(app.prefs.show_header, !before);
}

#[test]
fn x_toggles_border() {
    let mut app = make_app_with_disks(sample_disks());
    let before = app.prefs.show_border;
    app.handle_key(make_key(KeyCode::Char('x')));
    assert_eq!(app.prefs.show_border, !before);
}

#[test]
fn m_toggles_compact_and_resets_mount_w() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.col_mount_w = 25;
    let before = app.prefs.compact;
    app.handle_key(make_key(KeyCode::Char('m')));
    assert_eq!(app.prefs.compact, !before);
    assert_eq!(app.prefs.col_mount_w, 0, "col_mount_w should reset on compact toggle");
}

#[test]
fn w_toggles_full_mount() {
    let mut app = make_app_with_disks(sample_disks());
    let before = app.prefs.full_mount;
    app.handle_key(make_key(KeyCode::Char('w')));
    assert_eq!(app.prefs.full_mount, !before);
}

// ─── Bar style cycling ──────────────────────────────────────────────────

#[test]
fn bar_style_cycles_correctly() {
    let mut app = make_app_with_disks(sample_disks());
    assert_eq!(app.prefs.bar_style, BarStyle::Gradient);
    app.handle_key(make_key(KeyCode::Char('b')));
    assert_eq!(app.prefs.bar_style, BarStyle::Solid);
    app.handle_key(make_key(KeyCode::Char('b')));
    assert_eq!(app.prefs.bar_style, BarStyle::Thin);
    app.handle_key(make_key(KeyCode::Char('b')));
    assert_eq!(app.prefs.bar_style, BarStyle::Ascii);
    app.handle_key(make_key(KeyCode::Char('b')));
    assert_eq!(app.prefs.bar_style, BarStyle::Gradient);
}

// ─── Unit mode cycling ──────────────────────────────────────────────────

#[test]
fn unit_mode_cycles_correctly() {
    let mut app = make_app_with_disks(sample_disks());
    assert_eq!(app.prefs.unit_mode, UnitMode::Human);
    app.handle_key(make_key(KeyCode::Char('i')));
    assert_eq!(app.prefs.unit_mode, UnitMode::GiB);
    app.handle_key(make_key(KeyCode::Char('i')));
    assert_eq!(app.prefs.unit_mode, UnitMode::MiB);
    app.handle_key(make_key(KeyCode::Char('i')));
    assert_eq!(app.prefs.unit_mode, UnitMode::Bytes);
    app.handle_key(make_key(KeyCode::Char('i')));
    assert_eq!(app.prefs.unit_mode, UnitMode::Human);
}

// ─── Threshold cycling ──────────────────────────────────────────────────

#[test]
fn warn_threshold_cycles() {
    let mut app = make_app_with_disks(sample_disks());
    assert_eq!(app.prefs.thresh_warn, 70);
    app.handle_key(make_key(KeyCode::Char('t')));
    assert_eq!(app.prefs.thresh_warn, 80);
    app.handle_key(make_key(KeyCode::Char('t')));
    assert_eq!(app.prefs.thresh_warn, 50);
    app.handle_key(make_key(KeyCode::Char('t')));
    assert_eq!(app.prefs.thresh_warn, 60);
    app.handle_key(make_key(KeyCode::Char('t')));
    assert_eq!(app.prefs.thresh_warn, 70);
}

#[test]
fn crit_threshold_cycles() {
    let mut app = make_app_with_disks(sample_disks());
    assert_eq!(app.prefs.thresh_crit, 90);
    app.handle_key(make_key(KeyCode::Char('T')));
    assert_eq!(app.prefs.thresh_crit, 95);
    app.handle_key(make_key(KeyCode::Char('T')));
    assert_eq!(app.prefs.thresh_crit, 80);
    app.handle_key(make_key(KeyCode::Char('T')));
    assert_eq!(app.prefs.thresh_crit, 85);
    app.handle_key(make_key(KeyCode::Char('T')));
    assert_eq!(app.prefs.thresh_crit, 90);
}

// ─── Refresh rate cycling ───────────────────────────────────────────────

#[test]
fn refresh_rate_cycles() {
    let mut app = make_app_with_disks(sample_disks());
    assert_eq!(app.prefs.refresh_rate, 1);
    app.handle_key(make_key(KeyCode::Char('f')));
    assert_eq!(app.prefs.refresh_rate, 2);
    app.handle_key(make_key(KeyCode::Char('f')));
    assert_eq!(app.prefs.refresh_rate, 5);
    app.handle_key(make_key(KeyCode::Char('f')));
    assert_eq!(app.prefs.refresh_rate, 10);
    app.handle_key(make_key(KeyCode::Char('f')));
    assert_eq!(app.prefs.refresh_rate, 1);
}

// ─── Sort mode keys n/u/s/r ─────────────────────────────────────────────

#[test]
fn sort_n_switches_to_name() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.sort_mode = SortMode::Size;
    app.handle_key(make_key(KeyCode::Char('n')));
    assert_eq!(app.prefs.sort_mode, SortMode::Name);
    assert!(!app.prefs.sort_rev);
}

#[test]
fn sort_n_toggles_reverse_if_already_name() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.sort_rev = false;
    app.handle_key(make_key(KeyCode::Char('n')));
    assert!(app.prefs.sort_rev);
    app.handle_key(make_key(KeyCode::Char('n')));
    assert!(!app.prefs.sort_rev);
}

#[test]
fn sort_u_switches_to_pct() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.sort_mode = SortMode::Name;
    app.handle_key(make_key(KeyCode::Char('u')));
    assert_eq!(app.prefs.sort_mode, SortMode::Pct);
    assert!(!app.prefs.sort_rev);
}

#[test]
fn sort_u_toggles_reverse_if_already_pct() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.sort_mode = SortMode::Pct;
    app.handle_key(make_key(KeyCode::Char('u')));
    assert!(app.prefs.sort_rev);
}

#[test]
fn sort_s_switches_to_size() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.sort_mode = SortMode::Name;
    app.handle_key(make_key(KeyCode::Char('s')));
    assert_eq!(app.prefs.sort_mode, SortMode::Size);
    assert!(!app.prefs.sort_rev);
}

#[test]
fn sort_r_toggles_reverse() {
    let mut app = make_app_with_disks(sample_disks());
    assert!(!app.prefs.sort_rev);
    app.handle_key(make_key(KeyCode::Char('r')));
    assert!(app.prefs.sort_rev);
    app.handle_key(make_key(KeyCode::Char('r')));
    assert!(!app.prefs.sort_rev);
}

// ─── Help overlay blocks other keys ─────────────────────────────────────

#[test]
fn help_overlay_blocks_navigation() {
    let mut app = make_app_with_disks(sample_disks());
    app.show_help = true;
    app.handle_key(make_key(KeyCode::Char('s'))); // sort
    // Sort should NOT change because help is shown
    assert_eq!(app.prefs.sort_mode, SortMode::Name);
    assert!(!app.show_help); // but help is dismissed
}

#[test]
fn help_dismiss_with_q() {
    let mut app = make_app_with_disks(sample_disks());
    app.show_help = true;
    app.handle_key(make_key(KeyCode::Char('q')));
    assert!(!app.show_help);
    assert!(!app.quit, "q in help should dismiss help, not quit");
}

// ─── Theme editor large steps (H/L) ────────────────────────────────────

#[test]
fn theme_editor_large_step_l() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    assert!(app.theme_edit.active);
    let before = app.theme_edit.colors[0];
    app.handle_key(make_key(KeyCode::Char('L')));
    assert_eq!(app.theme_edit.colors[0], before.wrapping_add(10));
}

#[test]
fn theme_editor_large_step_h() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    let before = app.theme_edit.colors[0];
    app.handle_key(make_key(KeyCode::Char('H')));
    assert_eq!(app.theme_edit.colors[0], before.wrapping_sub(10));
}

#[test]
fn theme_editor_slot_bounds() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    // Navigate past bottom
    for _ in 0..10 {
        app.handle_key(make_key(KeyCode::Char('j')));
    }
    assert_eq!(app.theme_edit.slot, 5); // max slot
    // Navigate past top
    for _ in 0..10 {
        app.handle_key(make_key(KeyCode::Char('k')));
    }
    assert_eq!(app.theme_edit.slot, 0);
}

#[test]
fn theme_editor_naming_esc_cancels() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    app.handle_key(make_key(KeyCode::Char('s'))); // enter naming
    assert!(app.theme_edit.naming);
    for c in "test".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    app.handle_key(make_key(KeyCode::Esc));
    assert!(!app.theme_edit.naming);
    assert!(app.theme_edit.active, "Should stay in editor after naming cancel");
    assert!(!app.prefs.custom_themes.contains_key("test"));
}

#[test]
fn theme_editor_naming_empty_name_does_not_save() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    app.handle_key(make_key(KeyCode::Char('s')));
    assert!(app.theme_edit.naming);
    // Enter immediately (empty name)
    app.handle_key(make_key(KeyCode::Enter));
    assert!(!app.theme_edit.active);
    assert!(app.prefs.custom_themes.is_empty());
}

#[test]
fn theme_editor_naming_backspace() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    app.handle_key(make_key(KeyCode::Char('s')));
    for c in "abc".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    app.handle_key(make_key(KeyCode::Backspace));
    assert_eq!(app.theme_edit.name, "ab");
}

#[test]
fn theme_editor_naming_left_right() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    app.handle_key(make_key(KeyCode::Char('s')));
    for c in "ab".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    assert_eq!(app.theme_edit.cursor, 2);
    app.handle_key(make_key(KeyCode::Left));
    assert_eq!(app.theme_edit.cursor, 1);
    app.handle_key(make_key(KeyCode::Right));
    assert_eq!(app.theme_edit.cursor, 2);
}

#[test]
fn theme_editor_naming_max_length() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    app.handle_key(make_key(KeyCode::Char('s')));
    // Type 25 characters — max is 20
    for _ in 0..25 {
        app.handle_key(make_key(KeyCode::Char('x')));
    }
    assert_eq!(app.theme_edit.name.len(), 20);
}

// ─── Drill-down: Enter on file does nothing ─────────────────────────────

#[test]
fn drill_enter_on_file_does_not_navigate() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/tmp".into()];
    app.drill.entries = vec![
        DirEntry { path: "/tmp/file.txt".into(), name: "file.txt".into(), size: 100, is_dir: false },
    ];
    app.drill.selected = 0;
    let path_len = app.drill.path.len();
    app.handle_key(make_key(KeyCode::Enter));
    assert_eq!(app.drill.path.len(), path_len, "Should not drill into a file");
}

#[test]
fn drill_enter_on_dir_navigates() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/tmp".into()];
    app.drill.entries = vec![
        DirEntry { path: "/tmp/subdir".into(), name: "subdir".into(), size: 100, is_dir: true },
    ];
    app.drill.selected = 0;
    app.handle_key(make_key(KeyCode::Enter));
    assert_eq!(app.drill.path.len(), 2);
    assert_eq!(app.drill.path[1], "/tmp/subdir");
    assert!(app.drill.scanning);
}

#[test]
fn drill_open_key_shows_status() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/tmp".into()];
    app.handle_key(make_key(KeyCode::Char('o')));
    assert!(app.status_msg.is_some());
    assert!(app.status_msg.as_ref().unwrap().0.contains("Opened"));
}

// ─── Drill-down: j/k bounds ────────────────────────────────────────────

#[test]
fn drill_j_clamps_at_end() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 10, is_dir: false },
        DirEntry { path: "/b".into(), name: "b".into(), size: 20, is_dir: false },
    ];
    app.drill.selected = 0;
    for _ in 0..10 {
        app.handle_key(make_key(KeyCode::Char('j')));
    }
    assert_eq!(app.drill.selected, 1);
}

#[test]
fn drill_k_clamps_at_zero() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 10, is_dir: false },
    ];
    app.drill.selected = 0;
    for _ in 0..5 {
        app.handle_key(make_key(KeyCode::Char('k')));
    }
    assert_eq!(app.drill.selected, 0);
}

#[test]
fn drill_empty_entries_j_does_nothing() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.mode = ViewMode::DrillDown;
    app.drill.path = vec!["/".into()];
    app.drill.entries = vec![];
    app.drill.selected = 0;
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.drill.selected, 0);
}

// ─── y key: copy mount path ─────────────────────────────────────────────

#[test]
fn y_key_without_selection_shows_message() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = None;
    app.handle_key(make_key(KeyCode::Char('y')));
    assert!(app.status_msg.is_some());
    assert!(app.status_msg.as_ref().unwrap().0.contains("Select a disk"));
}

// ─── ColorMode::next cycling ────────────────────────────────────────────

#[test]
fn color_mode_next_wraps() {
    let last = *ColorMode::ALL.last().unwrap();
    assert_eq!(last.next(), ColorMode::ALL[0]);
}

#[test]
fn color_mode_next_all_variants() {
    let mut mode = ColorMode::Default;
    for _ in 0..ColorMode::ALL.len() {
        mode = mode.next();
    }
    assert_eq!(mode, ColorMode::Default, "Should wrap around to start");
}

// ─── ColorMode::name covers all variants ────────────────────────────────

#[test]
fn color_mode_name_all_non_empty() {
    for &mode in ColorMode::ALL {
        let name = mode.name();
        assert!(!name.is_empty(), "Name for {:?} should not be empty", mode);
        assert!(name.len() >= 3, "Name '{}' for {:?} too short", name, mode);
    }
}

#[test]
fn color_mode_all_has_correct_count() {
    assert_eq!(ColorMode::ALL.len(), 30);
}

// ─── Network filesystem detection ───────────────────────────────────────

#[test]
fn is_network_fs_detects_nfs() {
    use storageshower::system::is_network_fs;
    assert!(is_network_fs("nfs"));
    assert!(is_network_fs("nfs4"));
    assert!(is_network_fs("cifs"));
    assert!(is_network_fs("smbfs"));
    assert!(is_network_fs("afp"));
    assert!(is_network_fs("fuse.sshfs"));
    assert!(is_network_fs("fuse.rclone"));
    assert!(is_network_fs("fuse.s3fs"));
    assert!(is_network_fs("9p"));
    assert!(is_network_fs("afs"));
    assert!(is_network_fs("ncp"));
}

#[test]
fn is_network_fs_rejects_local() {
    use storageshower::system::is_network_fs;
    assert!(!is_network_fs("ext4"));
    assert!(!is_network_fs("apfs"));
    assert!(!is_network_fs("xfs"));
    assert!(!is_network_fs("btrfs"));
    assert!(!is_network_fs("tmpfs"));
    assert!(!is_network_fs(""));
}

// ─── Gradient color ─────────────────────────────────────────────────────

#[test]
fn gradient_color_at_boundary_values() {
    use storageshower::ui::gradient_color_at;
    use ratatui::style::Color;
    // Should not panic at boundaries
    let c0 = gradient_color_at(0.0, ColorMode::Default);
    let c33 = gradient_color_at(0.33, ColorMode::Default);
    let c55 = gradient_color_at(0.55, ColorMode::Default);
    let c80 = gradient_color_at(0.80, ColorMode::Default);
    let c100 = gradient_color_at(1.0, ColorMode::Default);
    // Green at start, escalating through palette
    assert!(matches!(c0, Color::Indexed(_)));
    assert!(matches!(c33, Color::Indexed(_)));
    assert!(matches!(c55, Color::Indexed(_)));
    assert!(matches!(c80, Color::Indexed(_)));
    assert!(matches!(c100, Color::Indexed(_)));
}

#[test]
fn gradient_color_at_all_color_modes() {
    use storageshower::ui::gradient_color_at;
    for &mode in ColorMode::ALL {
        for frac in [0.0, 0.1, 0.33, 0.5, 0.55, 0.75, 0.80, 0.9, 1.0] {
            let _ = gradient_color_at(frac, mode); // should not panic
        }
    }
}

// ─── Palette and palette_for_prefs ──────────────────────────────────────

#[test]
fn palette_all_modes_return_indexed_colors() {
    use storageshower::ui::palette;
    use ratatui::style::Color;
    for &mode in ColorMode::ALL {
        let (a, b, c, d, e, f) = palette(mode);
        assert!(matches!(a, Color::Indexed(_)), "palette {:?} slot 0 not indexed", mode);
        assert!(matches!(b, Color::Indexed(_)));
        assert!(matches!(c, Color::Indexed(_)));
        assert!(matches!(d, Color::Indexed(_)));
        assert!(matches!(e, Color::Indexed(_)));
        assert!(matches!(f, Color::Indexed(_)));
    }
}

#[test]
fn palette_for_prefs_uses_custom_theme() {
    use storageshower::ui::palette_for_prefs;
    use ratatui::style::Color;
    let mut prefs = Prefs::default();
    prefs.custom_themes.insert("mine".into(), ThemeColors {
        blue: 100, green: 101, purple: 102, light_purple: 103, royal: 104, dark_purple: 105,
    });
    prefs.active_theme = Some("mine".into());
    let (a, b, c, d, e, f) = palette_for_prefs(&prefs);
    assert_eq!(a, Color::Indexed(100));
    assert_eq!(b, Color::Indexed(101));
    assert_eq!(c, Color::Indexed(102));
    assert_eq!(d, Color::Indexed(103));
    assert_eq!(e, Color::Indexed(104));
    assert_eq!(f, Color::Indexed(105));
}

#[test]
fn palette_for_prefs_missing_custom_falls_back() {
    use storageshower::ui::palette_for_prefs;
    let mut prefs = Prefs::default();
    prefs.active_theme = Some("nonexistent".into());
    prefs.color_mode = ColorMode::Purple;
    let p = palette_for_prefs(&prefs);
    let expected = storageshower::ui::palette(ColorMode::Purple);
    assert_eq!(p, expected, "Should fall back to builtin when custom theme missing");
}

// ─── CLI parsing edge cases ─────────────────────────────────────────────

#[test]
fn cli_all_bar_styles() {
    for style in ["gradient", "solid", "thin", "ascii"] {
        let cli = Cli::parse_from(["storageshower", "-b", style]);
        assert!(cli.bar_style.is_some());
    }
}

#[test]
fn cli_all_sort_modes() {
    for mode in ["name", "pct", "size"] {
        let cli = Cli::parse_from(["storageshower", "-s", mode]);
        assert!(cli.sort_mode.is_some());
    }
}

#[test]
fn cli_all_unit_modes() {
    for mode in ["human", "gib", "mib", "bytes"] {
        let cli = Cli::parse_from(["storageshower", "-u", mode]);
        assert!(cli.unit_mode.is_some());
    }
}

#[test]
fn cli_all_color_modes() {
    let names = [
        "default", "green", "blue", "purple", "amber", "cyan", "red",
        "sakura", "matrix", "sunset", "neon-noir", "chrome-heart",
        "blade-runner", "void-walker", "toxic-waste", "cyber-frost",
        "plasma-core", "steel-nerve", "dark-signal", "glitch-pop",
        "holo-shift", "night-city", "deep-net", "laser-grid",
        "quantum-flux", "bio-hazard", "darkwave", "overlock", "megacorp", "zaibatsu",
    ];
    for name in names {
        let cli = Cli::parse_from(["storageshower", "--color", name]);
        assert!(cli.color_mode.is_some(), "Failed to parse color mode: {}", name);
    }
}

#[test]
fn cli_column_width_flags() {
    let cli = Cli::parse_from(["storageshower", "--col-mount", "25", "--col-bar-end", "30", "--col-pct", "8"]);
    assert_eq!(cli.col_mount_w, Some(25));
    assert_eq!(cli.col_bar_end_w, Some(30));
    assert_eq!(cli.col_pct_w, Some(8));
}

#[test]
fn cli_boolean_flag_pairs() {
    // --bars / --no-bars
    let cli = Cli::parse_from(["storageshower", "--no-bars"]);
    assert!(cli.no_bars);
    let cli = Cli::parse_from(["storageshower", "--bars", "--no-bars"]);
    assert!(cli.no_bars);

    // --border / --no-border
    let cli = Cli::parse_from(["storageshower", "--no-border"]);
    assert!(cli.no_border);

    // --header / --no-header
    let cli = Cli::parse_from(["storageshower", "--no-header"]);
    assert!(cli.no_header);

    // --compact / --no-compact
    let cli = Cli::parse_from(["storageshower", "--compact"]);
    assert!(cli.compact);

    // --used / --no-used
    let cli = Cli::parse_from(["storageshower", "--no-used"]);
    assert!(cli.no_used);

    // --full-mount / --no-full-mount
    let cli = Cli::parse_from(["storageshower", "--full-mount"]);
    assert!(cli.full_mount);

    // --virtual / --no-virtual
    let cli = Cli::parse_from(["storageshower", "--no-virtual"]);
    assert!(cli.no_virtual);
}

#[test]
fn cli_config_path() {
    let cli = Cli::parse_from(["storageshower", "-c", "/tmp/my.conf"]);
    assert_eq!(cli.config, Some("/tmp/my.conf".into()));
}

#[test]
fn cli_list_colors_flag() {
    let cli = Cli::parse_from(["storageshower", "--list-colors"]);
    assert!(cli.list_colors);
}

#[test]
fn cli_theme_flag() {
    let cli = Cli::parse_from(["storageshower", "--theme", "custom1"]);
    assert_eq!(cli.theme, Some("custom1".into()));
}

#[test]
fn cli_help_and_version_flags() {
    let cli = Cli::parse_from(["storageshower", "-h"]);
    assert!(cli.help);
    let cli = Cli::parse_from(["storageshower", "-V"]);
    assert!(cli.version);
}

// ─── CLI apply_to does not mutate when flags absent ─────────────────────

#[test]
fn cli_apply_to_preserves_all_defaults() {
    let cli = Cli::parse_from(["storageshower"]);
    let mut prefs = Prefs::default();
    prefs.sort_mode = SortMode::Size;
    prefs.bar_style = BarStyle::Thin;
    prefs.color_mode = ColorMode::Purple;
    prefs.unit_mode = UnitMode::GiB;
    prefs.thresh_warn = 55;
    prefs.thresh_crit = 88;
    prefs.refresh_rate = 7;
    prefs.compact = true;
    prefs.full_mount = true;
    prefs.show_bars = false;
    prefs.show_border = false;
    prefs.show_header = false;
    prefs.show_used = false;
    prefs.col_mount_w = 30;
    prefs.col_bar_end_w = 25;
    prefs.col_pct_w = 8;
    cli.apply_to(&mut prefs);
    assert_eq!(prefs.sort_mode, SortMode::Size);
    assert_eq!(prefs.bar_style, BarStyle::Thin);
    assert_eq!(prefs.color_mode, ColorMode::Purple);
    assert_eq!(prefs.unit_mode, UnitMode::GiB);
    assert_eq!(prefs.thresh_warn, 55);
    assert_eq!(prefs.thresh_crit, 88);
    assert_eq!(prefs.refresh_rate, 7);
    assert!(prefs.compact);
    assert!(prefs.full_mount);
    assert!(!prefs.show_bars);
    assert!(!prefs.show_border);
    assert!(!prefs.show_header);
    assert!(!prefs.show_used);
    assert_eq!(prefs.col_mount_w, 30);
    assert_eq!(prefs.col_bar_end_w, 25);
    assert_eq!(prefs.col_pct_w, 8);
}

// ─── Prefs custom themes round-trip ─────────────────────────────────────

#[test]
fn prefs_multiple_custom_themes_roundtrip() {
    let mut prefs = Prefs::default();
    prefs.custom_themes.insert("theme1".into(), ThemeColors {
        blue: 10, green: 20, purple: 30, light_purple: 40, royal: 50, dark_purple: 60,
    });
    prefs.custom_themes.insert("theme2".into(), ThemeColors {
        blue: 100, green: 110, purple: 120, light_purple: 130, royal: 140, dark_purple: 150,
    });
    let s = toml::to_string_pretty(&prefs).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.custom_themes.len(), 2);
    assert_eq!(q.custom_themes["theme1"].blue, 10);
    assert_eq!(q.custom_themes["theme2"].blue, 100);
}

#[test]
fn prefs_bookmarks_roundtrip() {
    let mut prefs = Prefs::default();
    prefs.bookmarks = vec!["/".into(), "/home".into(), "/data".into()];
    let s = toml::to_string_pretty(&prefs).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.bookmarks, vec!["/", "/home", "/data"]);
}

// ─── Load prefs from config with custom themes ──────────────────────────

#[test]
fn load_prefs_with_custom_themes_and_bookmarks() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("themes.conf");
    std::fs::write(&path, r#"
sort_mode = "Name"
sort_rev = false
show_local = false
refresh_rate = 1
bar_style = "Gradient"
color_mode = "Default"
thresh_warn = 70
thresh_crit = 90
show_bars = true
show_border = true
show_header = true
compact = false
show_used = true
full_mount = false
active_theme = "cyber"
bookmarks = ["/", "/home"]

[custom_themes.cyber]
blue = 42
green = 84
purple = 126
light_purple = 168
royal = 210
dark_purple = 252
"#).unwrap();
    let prefs = load_prefs_from(Some(path.to_str().unwrap()));
    assert_eq!(prefs.active_theme, Some("cyber".into()));
    assert!(prefs.custom_themes.contains_key("cyber"));
    assert_eq!(prefs.custom_themes["cyber"].blue, 42);
    assert_eq!(prefs.bookmarks, vec!["/", "/home"]);
}

// ─── Disk data with latency, I/O, SMART ─────────────────────────────────

#[test]
fn disk_entry_with_network_metadata() {
    let d = DiskEntry {
        mount: "/nfs".into(), used: 100, total: 200, pct: 50.0,
        kind: DiskKind::Unknown(-1), fs: "nfs4".into(),
        latency_ms: Some(15.3),
        io_read_rate: Some(1_048_576.0),
        io_write_rate: Some(524_288.0),
        smart_status: Some(SmartHealth::Verified),
    };
    assert_eq!(d.latency_ms, Some(15.3));
    assert_eq!(d.io_read_rate, Some(1_048_576.0));
    assert_eq!(d.smart_status, Some(SmartHealth::Verified));
    // Format helpers work with these values
    assert_eq!(format_latency(15.3), "15ms");
    assert_eq!(format_rate(1_048_576.0), "1.0M/s");
}

#[test]
fn disk_entry_with_optional_none_fields() {
    let d = DiskEntry {
        mount: "/local".into(), used: 50, total: 100, pct: 50.0,
        kind: DiskKind::SSD, fs: "apfs".into(),
        latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None,
    };
    assert!(d.latency_ms.is_none());
    assert!(d.io_read_rate.is_none());
    assert!(d.smart_status.is_none());
}

// ─── App: ensure_visible edge cases ─────────────────────────────────────

#[test]
fn ensure_visible_none_selected_does_nothing() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = None;
    app.scroll_offset = 5;
    app.ensure_visible(3);
    assert_eq!(app.scroll_offset, 5);
}

#[test]
fn ensure_visible_selected_equals_offset() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = Some(3);
    app.scroll_offset = 3;
    app.ensure_visible(5);
    assert_eq!(app.scroll_offset, 3, "No scroll needed when selected == offset");
}

#[test]
fn ensure_drill_visible_at_boundary() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.selected = 4;
    app.drill.scroll_offset = 2;
    app.ensure_drill_visible(3); // visible: 2,3,4 — selected=4 is at edge
    assert_eq!(app.drill.scroll_offset, 2);
}

// ─── App: all_themes sorting ────────────────────────────────────────────

#[test]
fn all_themes_custom_sorted_alphabetically() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.custom_themes.insert("zeta".into(), ThemeColors {
        blue: 1, green: 2, purple: 3, light_purple: 4, royal: 5, dark_purple: 6,
    });
    app.prefs.custom_themes.insert("alpha".into(), ThemeColors {
        blue: 1, green: 2, purple: 3, light_purple: 4, royal: 5, dark_purple: 6,
    });
    app.prefs.custom_themes.insert("middle".into(), ThemeColors {
        blue: 1, green: 2, purple: 3, light_purple: 4, royal: 5, dark_purple: 6,
    });
    let themes = app.all_themes();
    let custom_start = ColorMode::ALL.len();
    assert_eq!(themes[custom_start].0, "alpha");
    assert_eq!(themes[custom_start + 1].0, "middle");
    assert_eq!(themes[custom_start + 2].0, "zeta");
}

// ─── App: apply_selected_theme ──────────────────────────────────────────

#[test]
fn apply_selected_theme_builtin_via_chooser() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('c')));
    // Navigate to index 5 (Cyan)
    for _ in 0..5 {
        app.handle_key(make_key(KeyCode::Char('j')));
    }
    assert_eq!(app.prefs.color_mode, ColorMode::ALL[5]);
    app.handle_key(make_key(KeyCode::Enter));
    assert!(app.prefs.active_theme.is_none());
}

#[test]
fn apply_selected_theme_custom_via_chooser() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.custom_themes.insert("mytest".into(), ThemeColors {
        blue: 1, green: 2, purple: 3, light_purple: 4, royal: 5, dark_purple: 6,
    });
    app.handle_key(make_key(KeyCode::Char('c')));
    let idx = ColorMode::ALL.len(); // first custom theme
    app.theme_chooser.selected = idx;
    app.handle_key(make_key(KeyCode::Enter));
    assert_eq!(app.prefs.active_theme, Some("mytest".into()));
}

// ─── Drill current path ─────────────────────────────────────────────────

#[test]
fn drill_current_path_empty() {
    let app = make_app_with_disks(sample_disks());
    assert_eq!(app.drill_current_path(), "");
}

#[test]
fn drill_current_path_single() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.path = vec!["/home".into()];
    assert_eq!(app.drill_current_path(), "/home");
}

#[test]
fn drill_current_path_nested() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.path = vec!["/".into(), "/usr".into(), "/usr/bin".into()];
    assert_eq!(app.drill_current_path(), "/usr/bin");
}

// ─── Sort drill entries ─────────────────────────────────────────────────

#[test]
fn sort_drill_entries_by_name_case_insensitive() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.entries = vec![
        DirEntry { path: "/C".into(), name: "Charlie".into(), size: 10, is_dir: false },
        DirEntry { path: "/a".into(), name: "alpha".into(), size: 30, is_dir: false },
        DirEntry { path: "/B".into(), name: "bravo".into(), size: 20, is_dir: false },
    ];
    app.drill.sort = DrillSortMode::Name;
    app.drill.sort_rev = false;
    app.sort_drill_entries();
    assert_eq!(app.drill.entries[0].name, "alpha");
    assert_eq!(app.drill.entries[1].name, "bravo");
    assert_eq!(app.drill.entries[2].name, "Charlie");
}

#[test]
fn sort_drill_entries_by_size_desc() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 10, is_dir: false },
        DirEntry { path: "/b".into(), name: "b".into(), size: 30, is_dir: false },
        DirEntry { path: "/c".into(), name: "c".into(), size: 20, is_dir: false },
    ];
    app.drill.sort = DrillSortMode::Size;
    app.drill.sort_rev = false;
    app.sort_drill_entries();
    assert_eq!(app.drill.entries[0].size, 30);
    assert_eq!(app.drill.entries[1].size, 20);
    assert_eq!(app.drill.entries[2].size, 10);
}

#[test]
fn sort_drill_entries_reversed() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 30, is_dir: false },
        DirEntry { path: "/b".into(), name: "b".into(), size: 10, is_dir: false },
    ];
    app.drill.sort = DrillSortMode::Size;
    app.drill.sort_rev = true;
    app.sort_drill_entries();
    assert_eq!(app.drill.entries[0].size, 10); // reversed
}

#[test]
fn sort_drill_entries_resets_selection_and_scroll() {
    let mut app = make_app_with_disks(sample_disks());
    app.drill.entries = vec![
        DirEntry { path: "/a".into(), name: "a".into(), size: 10, is_dir: false },
    ];
    app.drill.selected = 5;
    app.drill.scroll_offset = 3;
    app.sort_drill_entries();
    assert_eq!(app.drill.selected, 0);
    assert_eq!(app.drill.scroll_offset, 0);
}

// ─── Hover zone without border ──────────────────────────────────────────

#[test]
fn hover_zone_no_border() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.show_border = false;
    // Without border, title is at row 0
    app.hover.pos = Some((10, 0));
    assert_eq!(app.hovered_zone(40), HoverZone::TitleBar);
}

#[test]
fn hover_zone_no_border_no_header() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.show_border = false;
    app.prefs.show_header = false;
    // No border, no header: title=0, first disk at row 2
    app.hover.pos = Some((10, 2));
    assert_eq!(app.hovered_zone(40), HoverZone::DiskRow(0));
}

// ─── Hovered disk index without border ──────────────────────────────────

#[test]
fn hovered_disk_index_no_border() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.show_border = false;
    // Without border: title=0, sep, header, sep, first disk at row 4
    app.hover.pos = Some((10, 4));
    assert_eq!(app.hovered_disk_index(), Some(0));
}

#[test]
fn hovered_disk_index_no_border_no_header() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.show_border = false;
    app.prefs.show_header = false;
    // No border, no header: first_disk_row = 0 + 2 + 0 = 2
    app.hover.pos = Some((10, 2));
    assert_eq!(app.hovered_disk_index(), Some(0));
}

// ─── Scan directory with progress ───────────────────────────────────────

#[test]
fn scan_directory_with_progress_tracks_counters() {
    use storageshower::system::scan_directory_with_progress;
    let count = Arc::new(Mutex::new(0usize));
    let total = Arc::new(Mutex::new(0usize));
    let entries = scan_directory_with_progress("/tmp", Some(count.clone()), Some(total.clone()));
    let t = *total.lock().unwrap();
    let c = *count.lock().unwrap();
    // count should equal total after completion
    assert_eq!(c, t, "count should match total after scan");
    // entries should be sorted by size descending
    for w in entries.windows(2) {
        assert!(w[0].size >= w[1].size);
    }
}

// ─── Dir entry fields ───────────────────────────────────────────────────

#[test]
fn dir_entry_is_dir_flag() {
    let d = DirEntry { path: "/tmp/d".into(), name: "d".into(), size: 0, is_dir: true };
    assert!(d.is_dir);
    let f = DirEntry { path: "/tmp/f".into(), name: "f".into(), size: 100, is_dir: false };
    assert!(!f.is_dir);
}

// ─── Format helpers: edge/extreme values ────────────────────────────────

#[test]
fn format_rate_negative_treated_as_zero() {
    assert_eq!(format_rate(-1.0), "0B/s");
}

#[test]
fn format_rate_fractional_bytes() {
    assert_eq!(format_rate(0.5), "0B/s");
}

#[test]
fn format_rate_exact_boundaries() {
    assert_eq!(format_rate(1023.0), "1023B/s");
    assert_eq!(format_rate(1024.0), "1.0K/s");
    assert_eq!(format_rate(1_048_575.0), "1024.0K/s");
    assert_eq!(format_rate(1_048_576.0), "1.0M/s");
}

#[test]
fn format_latency_exact_boundary() {
    assert_eq!(format_latency(1.0), "1ms");
    assert_eq!(format_latency(999.0), "999ms");
    assert_eq!(format_latency(1000.0), "1.0s");
}

#[test]
fn format_bytes_human_half_values() {
    assert_eq!(format_bytes(1536, UnitMode::Human), "1.5K"); // 1.5K
    assert_eq!(format_bytes(1_572_864, UnitMode::Human), "1.5M"); // 1.5M
    assert_eq!(format_bytes(1_610_612_736, UnitMode::Human), "1.5G"); // 1.5G
}

// ─── Prefs: column width override behavior in sorting ───────────────────

#[test]
fn mount_col_width_with_custom_and_terminal_sizes() {
    let mut p = Prefs::default();
    p.col_mount_w = 50;
    // Should clamp to terminal width - 20
    let w = mount_col_width(60, &p);
    assert_eq!(w, 40); // 60 - 20 = 40
    let w = mount_col_width(200, &p);
    assert_eq!(w, 50); // fits
}

// ─── SysStats default ───────────────────────────────────────────────────

#[test]
fn sys_stats_default_load_avg_zero() {
    let s = SysStats::default();
    assert_eq!(s.load_avg, (0.0, 0.0, 0.0));
    assert_eq!(s.swap_used, 0);
    assert_eq!(s.swap_total, 0);
    assert!(s.kernel.is_empty());
    assert!(s.arch.is_empty());
    assert!(s.os_name.is_empty());
    assert!(s.os_version.is_empty());
}

// ─── Epoch to local: various timestamps ─────────────────────────────────

#[test]
fn epoch_to_local_recent_date() {
    // 2024-06-15 12:00:00 UTC = 1718452800
    let (y, mo, d, h, mi, s) = epoch_to_local(1718452800);
    assert!(y >= 2024 && y <= 2025);
    assert!((1..=12).contains(&mo));
    assert!((1..=31).contains(&d));
    assert!(h < 24);
    assert!(mi < 60);
    assert!(s < 60);
}

#[test]
fn epoch_to_local_large_timestamp() {
    // 2099-12-31 23:59:59 UTC = 4102444799
    let (y, mo, _d, h, mi, s) = epoch_to_local(4102444799);
    assert!(y >= 2099);
    assert!((1..=12).contains(&mo));
    assert!(h < 24);
    assert!(mi < 60);
    assert!(s < 60);
}

// ─── Alert: multiple disks crossing threshold ───────────────────────────

#[test]
fn alert_multiple_disks_crossing_threshold() {
    let disks = vec![
        DiskEntry { mount: "/".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        DiskEntry { mount: "/data".into(), used: 50, total: 100, pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
    ];
    let shared = Arc::new(Mutex::new((SysStats::default(), disks)));
    let mut app = App::new_default(shared.clone());
    app.prefs = Prefs::default();
    app.prefs.thresh_warn = 70;

    // Both under threshold
    app.refresh_data();
    assert!(app.alert.flash.is_none());

    // Both over threshold
    {
        let mut lock = shared.lock().unwrap();
        lock.1 = vec![
            DiskEntry { mount: "/".into(), used: 80, total: 100, pct: 80.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
            DiskEntry { mount: "/data".into(), used: 75, total: 100, pct: 75.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        ];
    }
    app.refresh_data();
    assert!(app.alert.flash.is_some());
    assert!(app.alert.mounts.contains("/"));
    assert!(app.alert.mounts.contains("/data"));
    let msg = &app.status_msg.as_ref().unwrap().0;
    assert!(msg.contains("/") && msg.contains("/data"));
}

// ─── Update sorted with empty disk list ─────────────────────────────────

#[test]
fn update_sorted_empty_disks() {
    let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
    let mut app = App::new_default(shared);
    app.update_sorted();
    assert!(app.sorted_disks().is_empty());
}

#[test]
fn update_sorted_single_disk() {
    let disks = vec![DiskEntry {
        mount: "/".into(), used: 50, total: 100, pct: 50.0,
        kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None,
        io_read_rate: None, io_write_rate: None, smart_status: None,
    }];
    let mut app = make_app_with_disks(disks);
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 1);
}

// ─── Navigation on empty disk list ──────────────────────────────────────

#[test]
fn navigate_empty_disks_does_not_crash() {
    let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
    let mut app = App::new_default(shared);
    app.test_mode = true;
    app.handle_key(make_key(KeyCode::Char('j')));
    app.handle_key(make_key(KeyCode::Char('k')));
    app.handle_key(make_key(KeyCode::Home));
    app.handle_key(make_key(KeyCode::End));
    app.handle_key(make_key(KeyCode::Char('G')));
    app.handle_key(KeyEvent {
        code: KeyCode::Char('d'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    assert!(app.selected.is_none() || app.selected == Some(0));
}

// ─── Combined workflow: filter + sort + bookmark + navigate ─────────────

#[test]
fn full_workflow_filter_sort_bookmark_navigate() {
    let mut app = make_app_with_disks(sample_disks());

    // Sort by size
    app.handle_key(make_key(KeyCode::Char('s')));
    app.update_sorted();

    // Select first and bookmark it
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.selected, Some(0));
    let bookmarked_mount = app.sorted_disks()[0].mount.clone();
    app.handle_key(make_key(KeyCode::Char('B')));
    assert!(app.prefs.bookmarks.contains(&bookmarked_mount));

    // Update sorted — bookmarked should float to top
    app.update_sorted();
    assert_eq!(app.sorted_disks()[0].mount, bookmarked_mount);

    // Filter
    app.handle_key(make_key(KeyCode::Char('/')));
    for c in "data".chars() {
        app.handle_key(make_key(KeyCode::Char(c)));
    }
    app.handle_key(make_key(KeyCode::Enter));
    app.update_sorted();
    assert!(app.sorted_disks().iter().all(|d| d.mount.contains("data")));

    // Clear filter
    app.handle_key(make_key(KeyCode::Char('0')));
    app.update_sorted();
    assert_eq!(app.sorted_disks().len(), 3); // sample_disks has 3 (show_all=true filters tmpfs)
}

// ─── Multiple bookmarks ordering preserved ──────────────────────────────

#[test]
fn bookmarks_preserve_relative_sort_within_groups() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.sort_rev = false;
    app.prefs.bookmarks = vec!["/home".into(), "/data".into()];
    app.update_sorted();
    let disks = app.sorted_disks();
    // Bookmarked group should be first 2 entries, in name order
    let bookmarked: Vec<&str> = disks.iter().take(2).map(|d| d.mount.as_str()).collect();
    assert_eq!(bookmarked, vec!["/data", "/home"]);
    // Non-bookmarked should follow in name order
    let rest: Vec<&str> = disks.iter().skip(2).map(|d| d.mount.as_str()).collect();
    let mut sorted_rest = rest.clone();
    sorted_rest.sort();
    assert_eq!(rest, sorted_rest);
}

// ─── Chrono now returns valid format ────────────────────────────────────

#[test]
fn chrono_now_returns_valid_format() {
    let (date, time) = chrono_now();
    // Date: YYYY-MM-DD
    assert_eq!(date.len(), 10);
    assert_eq!(&date[4..5], "-");
    assert_eq!(&date[7..8], "-");
    // Time: HH:MM:SS
    assert_eq!(time.len(), 8);
    assert_eq!(&time[2..3], ":");
    assert_eq!(&time[5..6], ":");
}

// ─── Scan directory on real path ────────────────────────────────────────

#[test]
fn scan_directory_root_has_entries() {
    let entries = scan_directory("/");
    assert!(!entries.is_empty(), "Root directory should have entries");
    // All entries should have non-empty names
    for e in &entries {
        assert!(!e.name.is_empty());
        assert!(!e.path.is_empty());
    }
}

#[test]
fn scan_directory_entries_have_valid_paths() {
    let entries = scan_directory("/tmp");
    for e in &entries {
        assert!(e.path.starts_with("/tmp/") || e.path == "/tmp",
            "Path '{}' should start with /tmp/", e.path);
    }
}

// ─── Collect disk entries: pct in valid range ───────────────────────────

#[test]
fn collect_disk_entries_pct_in_range() {
    let disks = collect_disk_entries();
    for d in &disks {
        assert!(d.pct >= 0.0 && d.pct <= 100.0,
            "Disk {} pct {} out of range", d.mount, d.pct);
    }
}

#[test]
fn collect_disk_entries_mount_not_empty() {
    let disks = collect_disk_entries();
    for d in &disks {
        assert!(!d.mount.is_empty(), "Mount should not be empty");
    }
}

// ─── ThemeColors fields ─────────────────────────────────────────────────

#[test]
fn theme_colors_clone_and_debug() {
    let t = ThemeColors {
        blue: 27, green: 48, purple: 135, light_purple: 141, royal: 63, dark_purple: 99,
    };
    let c = t.clone();
    assert_eq!(c.blue, 27);
    assert_eq!(c.green, 48);
    assert_eq!(c.purple, 135);
    assert_eq!(c.light_purple, 141);
    assert_eq!(c.royal, 63);
    assert_eq!(c.dark_purple, 99);
    let dbg = format!("{:?}", c);
    assert!(dbg.contains("ThemeColors"));
}

// ─── DrillSortMode ──────────────────────────────────────────────────────

#[test]
fn drill_sort_mode_equality() {
    assert_eq!(DrillSortMode::Size, DrillSortMode::Size);
    assert_eq!(DrillSortMode::Name, DrillSortMode::Name);
    assert_ne!(DrillSortMode::Size, DrillSortMode::Name);
}

// ─── Mouse scroll in theme chooser bounds ───────────────────────────────

#[test]
fn theme_chooser_scroll_up_at_top_stays() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('c')));
    assert_eq!(app.theme_chooser.selected, 0);
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::ScrollUp, column: 40, row: 12, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.theme_chooser.selected, 0);
}

#[test]
fn theme_chooser_scroll_down_at_bottom_stays() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('c')));
    let count = app.all_themes().len();
    app.theme_chooser.selected = count - 1;
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::ScrollDown, column: 40, row: 12, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert_eq!(app.theme_chooser.selected, count - 1);
}

// ─── Mouse up releases drag ────────────────────────────────────────────

#[test]
fn mouse_up_releases_drag() {
    let mut app = make_app_with_disks(sample_disks());
    app.drag = Some(DragTarget::MountSep);
    app.handle_mouse(
        MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), column: 30, row: 6, modifiers: KeyModifiers::NONE },
        80, 24,
    );
    assert!(app.drag.is_none());
}

// ─── Truncate mount: various widths with unicode ────────────────────────

#[test]
fn truncate_mount_ascii_exact_boundary() {
    let s = "/mnt/data";
    let r = truncate_mount(s, 9);
    assert_eq!(r, s);
}

#[test]
fn truncate_mount_ascii_one_over() {
    let s = "/mnt/data";
    let r = truncate_mount(s, 8);
    assert_eq!(r.chars().count(), 8);
    assert!(r.ends_with('\u{2026}'));
}

// ─── Format bytes: large values in different modes ──────────────────────

#[test]
fn format_bytes_large_in_all_modes() {
    let val = 5 * 1_099_511_627_776u64; // 5TB
    assert_eq!(format_bytes(val, UnitMode::Human), "5.0T");
    assert!(format_bytes(val, UnitMode::GiB).contains("G"));
    assert!(format_bytes(val, UnitMode::MiB).contains("M"));
    assert!(format_bytes(val, UnitMode::Bytes).ends_with("B"));
}

// ─── Collect sys stats on real system ───────────────────────────────────

#[test]
fn sys_stats_os_name_not_empty() {
    let sys = System::new_all();
    let stats = collect_sys_stats(&sys);
    assert!(!stats.os_name.is_empty(), "os_name should not be empty");
}

#[test]
fn sys_stats_mem_used_le_total() {
    let sys = System::new_all();
    let stats = collect_sys_stats(&sys);
    assert!(stats.mem_used <= stats.mem_total,
        "mem_used {} > mem_total {}", stats.mem_used, stats.mem_total);
}

// ─── Hover: hovered_zone with various term heights ──────────────────────

#[test]
fn hover_zone_small_terminal() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((5, 1));
    // Even with small terminal, should not panic
    let zone = app.hovered_zone(10);
    assert_eq!(zone, HoverZone::TitleBar);
}

#[test]
fn hover_zone_very_large_terminal() {
    let mut app = make_app_with_disks(sample_disks());
    app.hover.pos = Some((5, 1));
    let zone = app.hovered_zone(200);
    assert_eq!(zone, HoverZone::TitleBar);
}

// ─── Theme editor enter also enters naming mode ─────────────────────────

#[test]
fn theme_editor_enter_starts_naming() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('C')));
    app.handle_key(make_key(KeyCode::Enter));
    assert!(app.theme_edit.naming);
}

// ─── Misc: Esc deselects ────────────────────────────────────────────────

#[test]
fn esc_deselects() {
    let mut app = make_app_with_disks(sample_disks());
    app.selected = Some(1);
    app.handle_key(make_key(KeyCode::Esc));
    assert!(app.selected.is_none());
}

// ─── Misc: e key export (check it sets status) ─────────────────────────

#[test]
fn e_key_sets_export_status() {
    let mut app = make_app_with_disks(sample_disks());
    app.handle_key(make_key(KeyCode::Char('e')));
    assert!(app.status_msg.is_some());
    let msg = &app.status_msg.as_ref().unwrap().0;
    assert!(msg.contains("Export") || msg.contains("export"),
        "Expected export message, got: {}", msg);
}

// ─── Combined: rapid key presses ────────────────────────────────────────

#[test]
fn rapid_key_sequence_does_not_crash() {
    let mut app = make_app_with_disks(sample_disks());
    // Slam through a bunch of keys rapidly
    let keys = [
        KeyCode::Char('j'), KeyCode::Char('j'), KeyCode::Char('k'),
        KeyCode::Char('s'), KeyCode::Char('n'), KeyCode::Char('u'),
        KeyCode::Char('b'), KeyCode::Char('i'), KeyCode::Char('v'),
        KeyCode::Char('d'), KeyCode::Char('g'), KeyCode::Char('x'),
        KeyCode::Char('m'), KeyCode::Char('w'), KeyCode::Char('t'),
        KeyCode::Char('T'), KeyCode::Char('f'), KeyCode::Char('r'),
        KeyCode::Char('p'), KeyCode::Char('p'), // unpause
        KeyCode::Char('l'), KeyCode::Char('a'),
        KeyCode::Home, KeyCode::End,
        KeyCode::Char('G'), KeyCode::Char('0'),
    ];
    for &code in &keys {
        app.handle_key(make_key(code));
    }
    // Should not crash
    app.update_sorted();
    let _ = app.sorted_disks();
}

// ─── Stress: sort all modes with many disks ─────────────────────────────

#[test]
fn stress_sort_all_modes_1000_disks() {
    let mut disks = Vec::new();
    for i in 0..1000 {
        disks.push(DiskEntry {
            mount: format!("/mount_{:04}", i),
            used: (i as u64 * 1_000_000) % 1_000_000_000,
            total: 1_000_000_000,
            pct: (i as f64 / 10.0) % 100.0,
            kind: DiskKind::SSD,
            fs: "ext4".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        });
    }
    let mut app = make_app_with_disks(disks);

    for mode in [SortMode::Name, SortMode::Pct, SortMode::Size] {
        app.prefs.sort_mode = mode;
        for rev in [false, true] {
            app.prefs.sort_rev = rev;
            app.update_sorted();
            assert_eq!(app.sorted_disks().len(), 1000);
        }
    }
}

// ─── Stress: filter with special characters ─────────────────────────────

#[test]
fn filter_special_characters() {
    let mut app = make_app_with_disks(sample_disks());
    app.filter.text = "data/sub[1]".into();
    app.update_sorted();
    // Should not crash, just return empty
    assert!(app.sorted_disks().is_empty());
}

#[test]
fn filter_unicode_characters() {
    let mut app = make_app_with_disks(sample_disks());
    app.filter.text = "日本語".into();
    app.update_sorted();
    assert!(app.sorted_disks().is_empty());
}

// ─── App new with CLI ───────────────────────────────────────────────────

#[test]
fn app_new_with_full_cli_overrides() {
    let cli = Cli::parse_from([
        "storageshower", "-s", "pct", "-R", "-b", "thin",
        "--color", "amber", "-u", "mib", "-w", "50", "-C", "80",
        "-r", "10", "--no-bars", "--no-border", "--no-header",
        "--compact", "--full-mount", "--no-used", "--no-virtual",
    ]);
    let shared = Arc::new(Mutex::new((SysStats::default(), sample_disks())));
    let app = App::new(Arc::clone(&shared), &cli);
    assert_eq!(app.prefs.sort_mode, SortMode::Pct);
    assert!(app.prefs.sort_rev);
    assert_eq!(app.prefs.bar_style, BarStyle::Thin);
    assert_eq!(app.prefs.color_mode, ColorMode::Amber);
    assert_eq!(app.prefs.unit_mode, UnitMode::MiB);
    assert_eq!(app.prefs.thresh_warn, 50);
    assert_eq!(app.prefs.thresh_crit, 80);
    assert_eq!(app.prefs.refresh_rate, 10);
    assert!(!app.prefs.show_bars);
    assert!(!app.prefs.show_border);
    assert!(!app.prefs.show_header);
    assert!(app.prefs.compact);
    assert!(app.prefs.full_mount);
    assert!(!app.prefs.show_used);
}
