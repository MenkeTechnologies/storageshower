use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use sysinfo::DiskKind;

use crate::cli::Cli;
use crate::prefs::{load_prefs_from, save_prefs, Prefs};
use crate::system::scan_directory_with_progress;
use crate::types::*;

pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let candidates: &[&[&str]] = &[
        #[cfg(target_os = "macos")]
        &["pbcopy"],
        #[cfg(target_os = "linux")]
        &["wl-copy"],
        #[cfg(target_os = "linux")]
        &["xclip", "-selection", "clipboard"],
        #[cfg(target_os = "linux")]
        &["xsel", "--clipboard", "--input"],
    ];

    for cmd in candidates {
        let program = cmd[0];
        let args = &cmd[1..];
        if let Ok(mut child) = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(text.as_bytes());
            }
            if let Ok(status) = child.wait() {
                if status.success() {
                    return Ok(());
                }
            }
        }
    }

    Err("no clipboard tool found (pbcopy/wl-copy/xclip/xsel)".into())
}

#[derive(Default)]
pub struct AlertState {
    pub mounts: HashSet<String>,
    pub flash: Option<Instant>,
}

#[derive(Default)]
pub struct HoverState {
    pub pos: Option<(u16, u16)>,
    pub since: Option<Instant>,
    pub right_click: bool,
}

#[derive(Default)]
pub struct ThemeEditState {
    pub active: bool,
    pub colors: [u8; 6],
    pub slot: usize,
    pub naming: bool,
    pub name: String,
    pub cursor: usize,
}

#[derive(Default)]
pub struct FilterState {
    pub text: String,
    pub active: bool,
    pub buf: String,
    pub prev: String,
    pub cursor: usize,
}

pub struct DrillState {
    pub mode: ViewMode,
    pub sort: DrillSortMode,
    pub sort_rev: bool,
    pub path: Vec<String>,
    pub entries: Vec<DirEntry>,
    pub selected: usize,
    pub scroll_offset: usize,
    pub scanning: bool,
    pub scan_result: Arc<Mutex<Option<Vec<DirEntry>>>>,
    pub scan_count: Arc<Mutex<usize>>,
    pub scan_total: Arc<Mutex<usize>>,
}

impl Default for DrillState {
    fn default() -> Self {
        Self {
            mode: ViewMode::Disks,
            sort: DrillSortMode::Size,
            sort_rev: false,
            path: Vec::new(),
            entries: Vec::new(),
            selected: 0,
            scroll_offset: 0,
            scanning: false,
            scan_result: Arc::new(Mutex::new(None)),
            scan_count: Arc::new(Mutex::new(0)),
            scan_total: Arc::new(Mutex::new(0)),
        }
    }
}

pub struct ThemeChooser {
    pub active: bool,
    pub selected: usize,
    pub orig_color_mode: ColorMode,
    pub orig_active_theme: Option<String>,
}

impl Default for ThemeChooser {
    fn default() -> Self {
        Self {
            active: false,
            selected: 0,
            orig_color_mode: ColorMode::Default,
            orig_active_theme: None,
        }
    }
}

pub struct App {
    pub prefs: Prefs,
    pub disks: Vec<DiskEntry>,
    pub stats: SysStats,
    pub shared_stats: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>,
    pub paused: bool,
    pub show_help: bool,
    pub quit: bool,
    pub drag: Option<DragTarget>,
    pub selected: Option<usize>,
    pub scroll_offset: usize,
    pub status_msg: Option<(String, Instant)>,
    pub filter: FilterState,
    pub hover: HoverState,
    pub theme_edit: ThemeEditState,
    pub alert: AlertState,
    pub drill: DrillState,
    pub theme_chooser: ThemeChooser,
    pub test_mode: bool,
    pub sorted_cache: Vec<DiskEntry>,
}

impl App {
    pub fn new(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>, cli: &Cli) -> Self {
        let mut prefs = load_prefs_from(cli.config.as_deref());
        cli.apply_to(&mut prefs);
        Self::with_prefs(shared, prefs)
    }

    /// Create with default prefs (no CLI overrides).
    pub fn new_default(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>) -> Self {
        Self::with_prefs(shared, Prefs::default())
    }

    fn with_prefs(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>, prefs: Prefs) -> Self {
        let (stats, disks) = shared.lock().unwrap().clone();
        let mut app = Self {
            prefs,
            disks,
            stats,
            shared_stats: shared,
            paused: false,
            show_help: false,
            quit: false,
            selected: None,
            scroll_offset: 0,
            status_msg: None,
            drag: None,
            filter: FilterState::default(),
            hover: HoverState::default(),
            theme_edit: ThemeEditState::default(),
            alert: AlertState::default(),
            drill: DrillState::default(),
            theme_chooser: ThemeChooser::default(),
            test_mode: false,
            sorted_cache: Vec::new(),
        };
        app.update_sorted();
        app
    }

    pub fn refresh_data(&mut self) {
        // Check for completed drill-down scans
        if self.drill.scanning {
            let taken = self.drill.scan_result.lock().unwrap().take();
            if let Some(entries) = taken {
                self.drill.entries = entries;
                self.drill.scanning = false;
                self.sort_drill_entries();
            }
        }

        if self.paused {
            return;
        }
        let (stats, disks) = self.shared_stats.lock().unwrap().clone();
        self.stats = stats;
        self.disks = disks;
        self.update_sorted();

        // Check for newly crossed thresholds
        let warn = self.prefs.thresh_warn as f64;
        let mut new_alerts: Vec<String> = Vec::new();
        let mut current_alert_mounts = HashSet::new();
        for d in &self.disks {
            if d.pct >= warn {
                current_alert_mounts.insert(d.mount.clone());
                if !self.alert.mounts.contains(&d.mount) {
                    new_alerts.push(format!("{} {:.0}%", d.mount, d.pct));
                }
            }
        }
        if !new_alerts.is_empty() {
            self.alert.flash = Some(Instant::now());
            let msg = format!("\u{26A0} ALERT: {}", new_alerts.join(", "));
            self.status_msg = Some((msg, Instant::now()));
            // Terminal bell
            print!("\x07");
        }
        self.alert.mounts = current_alert_mounts;
    }

    pub fn start_drill_scan(&mut self, path: &str) {
        self.drill.scanning = true;
        self.drill.entries.clear();
        *self.drill.scan_count.lock().unwrap() = 0;
        *self.drill.scan_total.lock().unwrap() = 0;
        let result = Arc::clone(&self.drill.scan_result);
        let count = Arc::clone(&self.drill.scan_count);
        let total = Arc::clone(&self.drill.scan_total);
        let path = path.to_string();
        std::thread::spawn(move || {
            let entries = scan_directory_with_progress(&path, Some(count), Some(total));
            *result.lock().unwrap() = Some(entries);
        });
    }

    /// List all available themes: builtins then custom, as (key, display_name) pairs.
    pub fn all_themes(&self) -> Vec<(String, String)> {
        let mut themes: Vec<(String, String)> = Vec::new();
        for &mode in ColorMode::ALL {
            themes.push((format!("{:?}", mode).to_lowercase(), mode.name().to_string()));
        }
        let mut custom_names: Vec<String> = self.prefs.custom_themes.keys().cloned().collect();
        custom_names.sort();
        for name in custom_names {
            themes.push((name.clone(), name));
        }
        themes
    }

    /// Apply the currently selected theme in the chooser (live preview).
    pub(crate) fn apply_selected_theme(&mut self) {
        let themes = self.all_themes();
        if let Some((key, _)) = themes.get(self.theme_chooser.selected) {
            let mut found_builtin = false;
            for &mode in ColorMode::ALL {
                if format!("{:?}", mode).to_lowercase() == *key {
                    self.prefs.color_mode = mode;
                    self.prefs.active_theme = None;
                    found_builtin = true;
                    break;
                }
            }
            if !found_builtin {
                self.prefs.active_theme = Some(key.clone());
            }
        }
    }

    pub fn hover_ready(&self) -> bool {
        self.hover.since
            .map(|t| {
                let elapsed = t.elapsed().as_millis();
                let visible = elapsed >= 1000;
                // Auto-hide after 4s (3s visible) unless triggered by right-click
                let expired = !self.hover.right_click && elapsed >= 4000;
                visible && !expired
            })
            .unwrap_or(false)
    }

    pub fn hovered_zone(&self, term_h: u16) -> HoverZone {
        let (_, y) = match self.hover.pos {
            Some(pos) => pos,
            None => return HoverZone::None,
        };
        let title_row: u16 = if self.prefs.show_border { 1 } else { 0 };
        let first_disk_row = title_row + 2
            + if self.prefs.show_header { 2 } else { 0 };
        let footer_rows: u16 = 2 + if self.prefs.show_border { 1 } else { 0 };
        let footer_row = term_h.saturating_sub(footer_rows) + 1;

        if y == title_row {
            return HoverZone::TitleBar;
        }
        if y >= footer_row && y < term_h.saturating_sub(if self.prefs.show_border { 1 } else { 0 }) {
            return HoverZone::FooterBar;
        }
        if y >= first_disk_row {
            let idx = (y - first_disk_row) as usize;
            let count = self.sorted_disks().len();
            if idx < count {
                return HoverZone::DiskRow(idx);
            }
        }
        HoverZone::None
    }

    pub fn hovered_disk_index(&self) -> Option<usize> {
        let (_, y) = self.hover.pos?;
        let first_disk_row: u16 = if self.prefs.show_border { 1 } else { 0 }
            + 2
            + if self.prefs.show_header { 2 } else { 0 };
        if y < first_disk_row {
            return None;
        }
        let idx = (y - first_disk_row) as usize;
        let count = self.sorted_disks().len();
        if idx < count { Some(idx) } else { None }
    }

    pub fn hovered_drill_index(&self) -> Option<usize> {
        let (_, y) = self.hover.pos?;
        // Drill-down layout: border(0/1) + breadcrumb + separator + header + separator = first entry row
        let first_entry_row: u16 = if self.prefs.show_border { 1 } else { 0 } + 4;
        if y < first_entry_row {
            return None;
        }
        let idx = (y - first_entry_row) as usize;
        if idx < self.drill.entries.len() { Some(idx) } else { None }
    }

    pub fn drill_current_path(&self) -> String {
        self.drill.path.last().cloned().unwrap_or_default()
    }

    pub fn sort_drill_entries(&mut self) {
        match self.drill.sort {
            DrillSortMode::Size => self.drill.entries.sort_by(|a, b| b.size.cmp(&a.size)),
            DrillSortMode::Name => self.drill.entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        }
        if self.drill.sort_rev {
            self.drill.entries.reverse();
        }
        self.drill.selected = 0;
        self.drill.scroll_offset = 0;
    }

    /// Adjust scroll_offset so `selected` is visible in `visible_rows` window.
    pub fn ensure_visible(&mut self, visible_rows: usize) {
        if let Some(sel) = self.selected {
            if sel < self.scroll_offset {
                self.scroll_offset = sel;
            } else if sel >= self.scroll_offset + visible_rows {
                self.scroll_offset = sel.saturating_sub(visible_rows - 1);
            }
        }
    }

    /// Adjust drill_scroll_offset so drill_selected is visible.
    pub fn ensure_drill_visible(&mut self, visible_rows: usize) {
        if self.drill.selected < self.drill.scroll_offset {
            self.drill.scroll_offset = self.drill.selected;
        } else if self.drill.selected >= self.drill.scroll_offset + visible_rows {
            self.drill.scroll_offset = self.drill.selected.saturating_sub(visible_rows - 1);
        }
    }

    /// Recompute the cached sorted/filtered disk list.
    /// Call this after changing disks, prefs, or filter state.
    pub fn update_sorted(&mut self) {
        let mut ds: Vec<DiskEntry> = self.disks.clone();
        if !self.prefs.show_all {
            ds.retain(|d| {
                d.total > 0
                    && !d.mount.starts_with("/sys")
                    && !d.mount.starts_with("/proc")
                    && !d.mount.starts_with("/dev/shm")
                    && !d.mount.starts_with("/run")
                    && !d.mount.starts_with("/snap")
                    && d.fs != "tmpfs"
                    && d.fs != "devtmpfs"
                    && d.fs != "squashfs"
                    && d.fs != "overlay"
                    && d.fs != "devfs"
                    && d.fs != "map"
                    && d.fs != "autofs"
            });
        }
        if self.prefs.show_local {
            ds.retain(|d| {
                matches!(d.kind, DiskKind::HDD | DiskKind::SSD)
                    || d.total > 0
            });
        }
        if !self.filter.text.is_empty() {
            let f = self.filter.text.to_lowercase();
            ds.retain(|d| d.mount.to_lowercase().contains(&f));
        }
        match self.prefs.sort_mode {
            SortMode::Name => ds.sort_by(|a, b| a.mount.cmp(&b.mount)),
            SortMode::Pct => ds.sort_by(|a, b| a.pct.partial_cmp(&b.pct).unwrap_or(std::cmp::Ordering::Equal)),
            SortMode::Size => ds.sort_by(|a, b| a.total.cmp(&b.total)),
        }
        if self.prefs.sort_rev {
            ds.reverse();
        }
        if !self.prefs.bookmarks.is_empty() {
            ds.sort_by_key(|d| if self.prefs.bookmarks.contains(&d.mount) { 0 } else { 1 });
        }
        self.sorted_cache = ds;
    }

    /// Return the cached sorted/filtered disk list.
    pub fn sorted_disks(&self) -> &[DiskEntry] {
        &self.sorted_cache
    }

    pub fn save(&self) {
        if self.test_mode { return; }
        save_prefs(&self.prefs);
    }
    // handle_key and handle_mouse are in the input module.
}

// ─── Column width helpers (re-exported from columns module) ────────────────
pub use crate::columns::{mount_col_width, right_col_width, right_col_width_static};

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;
    use crate::testutil::*;


    // ── Sorting ────────────────────────────────────────────

    #[test]
    fn sorted_disks_by_name() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Name;
        app.prefs.sort_rev = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        let names: Vec<&str> = disks.iter().map(|d| d.mount.as_str()).collect();
        assert_eq!(names, vec!["/", "/data", "/home", "/tmp"]);
    }

    #[test]
    fn sorted_disks_by_name_reversed() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Name;
        app.prefs.sort_rev = true;
        app.update_sorted();
        let disks = app.sorted_disks();
        let names: Vec<&str> = disks.iter().map(|d| d.mount.as_str()).collect();
        assert_eq!(names, vec!["/tmp", "/home", "/data", "/"]);
    }

    #[test]
    fn sorted_disks_by_pct() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Pct;
        app.prefs.sort_rev = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        let pcts: Vec<f64> = disks.iter().map(|d| d.pct).collect();
        assert!(pcts.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn sorted_disks_by_size() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Size;
        app.prefs.sort_rev = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        let sizes: Vec<u64> = disks.iter().map(|d| d.total).collect();
        assert!(sizes.windows(2).all(|w| w[0] <= w[1]));
    }

    // ── Filtering ──────────────────────────────────────────

    #[test]
    fn sorted_disks_filter() {
        let mut app = test_app();
        app.filter.text = "home".into();
        app.update_sorted();
        let disks = app.sorted_disks();
        assert_eq!(disks.len(), 1);
        assert_eq!(disks[0].mount, "/home");
    }

    #[test]
    fn sorted_disks_filter_case_insensitive() {
        let mut app = test_app();
        app.filter.text = "HOME".into();
        app.update_sorted();
        let disks = app.sorted_disks();
        assert_eq!(disks.len(), 1);
        assert_eq!(disks[0].mount, "/home");
    }

    #[test]
    fn sorted_disks_filter_no_match() {
        let mut app = test_app();
        app.filter.text = "nonexistent".into();
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(disks.is_empty());
    }

    #[test]
    fn sorted_disks_show_all_off_filters_tmpfs() {
        let mut app = test_app();
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.fs == "tmpfs"));
    }

    // ── refresh_data while paused ──────────────────────────

    #[test]
    fn refresh_data_paused_does_nothing() {
        let mut app = test_app();
        app.paused = true;
        let old_disks_len = app.disks.len();
        app.refresh_data();
        assert_eq!(app.disks.len(), old_disks_len);
    }

    // ── show_all=false filters various virtual fs ─────────

    #[test]
    fn show_all_off_filters_sys() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/sys/kernel".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "sysfs".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.mount.starts_with("/sys")));
    }

    #[test]
    fn show_all_off_filters_proc() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/proc".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "proc".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.mount.starts_with("/proc")));
    }

    #[test]
    fn show_all_off_filters_dev_shm() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/dev/shm".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "tmpfs".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.mount.starts_with("/dev/shm")));
    }

    #[test]
    fn show_all_off_filters_run() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/run/lock".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "tmpfs".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.mount.starts_with("/run")));
    }

    #[test]
    fn show_all_off_filters_snap() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/snap/core".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "squashfs".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.mount.starts_with("/snap")));
    }

    #[test]
    fn show_all_off_filters_overlay() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/var/lib/docker".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "overlay".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.fs == "overlay"));
    }

    #[test]
    fn show_all_off_filters_devtmpfs() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/dev".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "devtmpfs".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.fs == "devtmpfs"));
    }

    #[test]
    fn show_all_off_filters_devfs() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/dev".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "devfs".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.fs == "devfs"));
    }

    #[test]
    fn show_all_off_filters_autofs() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/net".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "autofs".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.fs == "autofs"));
    }

    #[test]
    fn show_all_off_filters_map() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/net".into(), used: 0, total: 100, pct: 0.0,
            kind: DiskKind::Unknown(-1), fs: "map".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.fs == "map"));
    }

    #[test]
    fn show_all_off_filters_zero_total() {
        let mut app = test_app();
        app.disks.push(DiskEntry {
            mount: "/empty".into(), used: 0, total: 0, pct: 0.0,
            kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.prefs.show_all = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert!(!disks.iter().any(|d| d.mount == "/empty"));
    }

    // ── Sort by pct reversed ──────────────────────────────

    #[test]
    fn sorted_disks_by_pct_reversed() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Pct;
        app.prefs.sort_rev = true;
        app.update_sorted();
        let disks = app.sorted_disks();
        let pcts: Vec<f64> = disks.iter().map(|d| d.pct).collect();
        assert!(pcts.windows(2).all(|w| w[0] >= w[1]));
    }

    #[test]
    fn sorted_disks_by_size_reversed() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Size;
        app.prefs.sort_rev = true;
        app.update_sorted();
        let disks = app.sorted_disks();
        let sizes: Vec<u64> = disks.iter().map(|d| d.total).collect();
        assert!(sizes.windows(2).all(|w| w[0] >= w[1]));
    }

    // ── Filter + sort combined ────────────────────────────

    #[test]
    fn filter_and_sort_combined() {
        let mut app = test_app();
        // Add more disks with 'a' in name
        app.disks.push(DiskEntry {
            mount: "/data2".into(), used: 200_000_000_000, total: 400_000_000_000,
            pct: 50.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None,
            io_read_rate: None, io_write_rate: None, smart_status: None,
        });
        app.filter.text = "data".into();
        app.prefs.sort_mode = SortMode::Size;
        app.prefs.sort_rev = false;
        app.update_sorted();
        let disks = app.sorted_disks();
        assert_eq!(disks.len(), 2);
        assert!(disks[0].total <= disks[1].total);
    }

    // ── Bookmarks ──────────────────────────────────────────

    #[test]
    fn bookmark_toggle_on_selected() {
        let mut app = test_app();
        app.selected = Some(0);
        assert!(app.prefs.bookmarks.is_empty());
        app.handle_key(make_key(KeyCode::Char('B')));
        assert_eq!(app.prefs.bookmarks, vec!["/"]);
        // Toggle off
        app.handle_key(make_key(KeyCode::Char('B')));
        assert!(app.prefs.bookmarks.is_empty());
    }

    #[test]
    fn bookmark_pins_to_top() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Name;
        app.prefs.sort_rev = false;
        app.update_sorted();
        // Without bookmark, "/" is first alphabetically
        let disks = app.sorted_disks();
        assert_eq!(disks[0].mount, "/");

        // Bookmark "/home" — it should appear first
        app.prefs.bookmarks.push("/home".into());
        app.update_sorted();
        let disks = app.sorted_disks();
        assert_eq!(disks[0].mount, "/home");
    }

    #[test]
    fn bookmark_no_selection_shows_message() {
        let mut app = test_app();
        app.selected = None;
        app.handle_key(make_key(KeyCode::Char('B')));
        assert!(app.prefs.bookmarks.is_empty());
        assert!(app.status_msg.is_some());
    }
}
