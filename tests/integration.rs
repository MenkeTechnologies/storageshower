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
    assert!(app.filter.active);
    // Type "/"  — but "/" also triggers filter mode, so we need a different char
    app.handle_key(make_key(KeyCode::Char('d')));
    app.handle_key(make_key(KeyCode::Char('a')));
    app.handle_key(make_key(KeyCode::Char('t')));
    app.handle_key(make_key(KeyCode::Char('a')));
    app.handle_key(make_key(KeyCode::Enter));
    assert!(!app.filter.active);
    assert_eq!(app.filter.text, "data");

    let filtered = app.sorted_disks();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].mount, "/data");

    // Navigate: select first
    app.handle_key(make_key(KeyCode::Char('j')));
    assert_eq!(app.selected, Some(0));

    // Clear filter
    app.handle_key(make_key(KeyCode::Char('0')));
    assert!(app.filter.text.is_empty());
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
    let sorted = app.sorted_disks();
    assert_eq!(sorted.len(), 500);
    assert!(sorted.windows(2).all(|w| w[0].pct <= w[1].pct));

    // Filter
    app.filter.text = "mount_00".into();
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
    let disks = app.sorted_disks();
    assert_eq!(disks[0].mount, "/home", "Bookmarked disk should be first");
}

#[test]
fn multiple_bookmarks_all_pinned() {
    let mut app = make_app_with_disks(sample_disks());
    app.prefs.sort_mode = SortMode::Name;
    app.prefs.bookmarks = vec!["/home".into(), "/data".into()];
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
    let cli = Cli::parse_from(["storageshower", "--export-theme", "-c", "purple"]);
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
