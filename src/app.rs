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
            .map(|t| t.elapsed().as_millis() >= 1000)
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
    use crossterm::event::{
        KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
        MouseButton, MouseEvent, MouseEventKind,
    };
    use std::sync::{Arc, Mutex};

    fn make_key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    fn make_ctrl_key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    fn test_disks() -> Vec<DiskEntry> {
        vec![
            DiskEntry { mount: "/".into(), used: 50_000_000_000, total: 100_000_000_000, pct: 50.0, kind: DiskKind::SSD, fs: "apfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
            DiskEntry { mount: "/home".into(), used: 80_000_000_000, total: 200_000_000_000, pct: 40.0, kind: DiskKind::SSD, fs: "ext4".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
            DiskEntry { mount: "/data".into(), used: 900_000_000_000, total: 1_000_000_000_000, pct: 90.0, kind: DiskKind::HDD, fs: "xfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
            DiskEntry { mount: "/tmp".into(), used: 100_000, total: 500_000_000, pct: 0.02, kind: DiskKind::Unknown(-1), fs: "tmpfs".into(), latency_ms: None, io_read_rate: None, io_write_rate: None, smart_status: None },
        ]
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

    // ── Column width helpers ───────────────────────────────

    #[test]
    fn mount_col_width_default() {
        let p = Prefs::default();
        let w = mount_col_width(120, &p);
        assert_eq!(w, 40); // 120/3
    }

    #[test]
    fn mount_col_width_compact() {
        let mut p = Prefs::default();
        p.compact = true;
        assert_eq!(mount_col_width(120, &p), 16);
    }

    #[test]
    fn mount_col_width_custom() {
        let mut p = Prefs::default();
        p.col_mount_w = 25;
        assert_eq!(mount_col_width(120, &p), 25);
    }

    #[test]
    fn mount_col_width_custom_clamped() {
        let mut p = Prefs::default();
        p.col_mount_w = 200;
        let w = mount_col_width(120, &p);
        assert!(w <= 100); // clamped to inner_w - 20
    }

    #[test]
    fn right_col_width_static_default() {
        let p = Prefs::default();
        assert_eq!(right_col_width_static(&p), 22); // show_used=true
    }

    #[test]
    fn right_col_width_static_no_used() {
        let mut p = Prefs::default();
        p.show_used = false;
        assert_eq!(right_col_width_static(&p), 7);
    }

    #[test]
    fn right_col_width_static_custom() {
        let mut p = Prefs::default();
        p.col_bar_end_w = 30;
        assert_eq!(right_col_width_static(&p), 30);
    }

    #[test]
    fn right_col_width_static_custom_min() {
        let mut p = Prefs::default();
        p.col_bar_end_w = 2; // below min
        assert_eq!(right_col_width_static(&p), 5);
    }

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

    // ── Key handling — quit ────────────────────────────────

    #[test]
    fn key_q_quits() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('q')));
        assert!(app.quit);
    }

    #[test]
    fn key_upper_q_quits() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('Q')));
        assert!(app.quit);
    }

    // ── Key handling — help ────────────────────────────────

    #[test]
    fn key_h_toggles_help() {
        let mut app = test_app();
        assert!(!app.show_help);
        app.handle_key(make_key(KeyCode::Char('h')));
        assert!(app.show_help);
        // dismiss with h
        app.handle_key(make_key(KeyCode::Char('h')));
        assert!(!app.show_help);
    }

    #[test]
    fn key_question_mark_opens_help() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('?')));
        assert!(app.show_help);
    }

    #[test]
    fn help_dismisses_with_esc() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Esc));
        assert!(!app.show_help);
    }

    #[test]
    fn help_dismisses_with_j() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Char('j')));
        assert!(!app.show_help);
    }

    // ── Key handling — pause ───────────────────────────────

    #[test]
    fn key_p_toggles_pause() {
        let mut app = test_app();
        assert!(!app.paused);
        app.handle_key(make_key(KeyCode::Char('p')));
        assert!(app.paused);
        app.handle_key(make_key(KeyCode::Char('p')));
        assert!(!app.paused);
    }

    // ── Key handling — sort ────────────────────────────────

    #[test]
    fn key_n_sorts_by_name() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Size;
        app.handle_key(make_key(KeyCode::Char('n')));
        assert_eq!(app.prefs.sort_mode, SortMode::Name);
        assert!(!app.prefs.sort_rev);
    }

    #[test]
    fn key_n_toggles_reverse_if_active() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Name;
        app.prefs.sort_rev = false;
        app.handle_key(make_key(KeyCode::Char('n')));
        assert!(app.prefs.sort_rev);
    }

    #[test]
    fn key_u_sorts_by_pct() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('u')));
        assert_eq!(app.prefs.sort_mode, SortMode::Pct);
    }

    #[test]
    fn key_s_sorts_by_size() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('s')));
        assert_eq!(app.prefs.sort_mode, SortMode::Size);
    }

    #[test]
    fn key_r_reverses_sort() {
        let mut app = test_app();
        assert!(!app.prefs.sort_rev);
        app.handle_key(make_key(KeyCode::Char('r')));
        assert!(app.prefs.sort_rev);
    }

    // ── Key handling — display toggles ─────────────────────

    #[test]
    fn key_b_cycles_bar_style() {
        let mut app = test_app();
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

    #[test]
    fn key_c_opens_theme_chooser() {
        let mut app = test_app();
        assert!(!app.theme_chooser.active);
        app.handle_key(make_key(KeyCode::Char('c')));
        assert!(app.theme_chooser.active);
        // Should pre-select current theme (Default = index 0)
        assert_eq!(app.theme_chooser.selected, 0);
    }

    #[test]
    fn theme_chooser_navigate_and_select() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('c'))); // open
        assert!(app.theme_chooser.active);

        // Navigate down
        app.handle_key(make_key(KeyCode::Char('j')));
        assert_eq!(app.theme_chooser.selected, 1);

        // Select with Enter
        app.handle_key(make_key(KeyCode::Enter));
        assert!(!app.theme_chooser.active);
        // Should have changed to second builtin theme
        assert_eq!(app.prefs.color_mode, ColorMode::ALL[1]);
    }

    #[test]
    fn theme_chooser_esc_cancels() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('c')));
        assert!(app.theme_chooser.active);
        app.handle_key(make_key(KeyCode::Esc));
        assert!(!app.theme_chooser.active);
        // Theme should not have changed
        assert_eq!(app.prefs.color_mode, ColorMode::Default);
    }

    #[test]
    fn key_i_cycles_unit_mode() {
        let mut app = test_app();
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

    #[test]
    fn key_v_toggles_bars() {
        let mut app = test_app();
        assert!(app.prefs.show_bars);
        app.handle_key(make_key(KeyCode::Char('v')));
        assert!(!app.prefs.show_bars);
    }

    #[test]
    fn key_x_toggles_border() {
        let mut app = test_app();
        assert!(app.prefs.show_border);
        app.handle_key(make_key(KeyCode::Char('x')));
        assert!(!app.prefs.show_border);
    }

    #[test]
    fn key_g_toggles_header() {
        let mut app = test_app();
        assert!(app.prefs.show_header);
        app.handle_key(make_key(KeyCode::Char('g')));
        assert!(!app.prefs.show_header);
    }

    #[test]
    fn key_d_toggles_show_used() {
        let mut app = test_app();
        assert!(app.prefs.show_used);
        app.handle_key(make_key(KeyCode::Char('d')));
        assert!(!app.prefs.show_used);
        assert_eq!(app.prefs.col_bar_end_w, 0); // reset
    }

    #[test]
    fn key_m_toggles_compact() {
        let mut app = test_app();
        assert!(!app.prefs.compact);
        app.handle_key(make_key(KeyCode::Char('m')));
        assert!(app.prefs.compact);
        assert_eq!(app.prefs.col_mount_w, 0); // reset
    }

    #[test]
    fn key_w_toggles_full_mount() {
        let mut app = test_app();
        assert!(!app.prefs.full_mount);
        app.handle_key(make_key(KeyCode::Char('w')));
        assert!(app.prefs.full_mount);
    }

    // ── Key handling — thresholds ──────────────────────────

    #[test]
    fn key_t_cycles_warn_threshold() {
        let mut app = test_app();
        assert_eq!(app.prefs.thresh_warn, 70);
        app.handle_key(make_key(KeyCode::Char('t')));
        assert_eq!(app.prefs.thresh_warn, 80);
        app.handle_key(make_key(KeyCode::Char('t')));
        assert_eq!(app.prefs.thresh_warn, 50);
    }

    #[test]
    fn key_upper_t_cycles_crit_threshold() {
        let mut app = test_app();
        assert_eq!(app.prefs.thresh_crit, 90);
        app.handle_key(make_key(KeyCode::Char('T')));
        assert_eq!(app.prefs.thresh_crit, 95);
        app.handle_key(make_key(KeyCode::Char('T')));
        assert_eq!(app.prefs.thresh_crit, 80);
    }

    #[test]
    fn key_f_cycles_refresh_rate() {
        let mut app = test_app();
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

    // ── Key handling — navigation ──────────────────────────

    #[test]
    fn key_j_selects_next() {
        let mut app = test_app();
        assert_eq!(app.selected, None);
        app.handle_key(make_key(KeyCode::Char('j')));
        assert_eq!(app.selected, Some(0));
        app.handle_key(make_key(KeyCode::Char('j')));
        assert_eq!(app.selected, Some(1));
    }

    #[test]
    fn key_k_selects_prev() {
        let mut app = test_app();
        app.selected = Some(2);
        app.handle_key(make_key(KeyCode::Char('k')));
        assert_eq!(app.selected, Some(1));
        app.handle_key(make_key(KeyCode::Char('k')));
        assert_eq!(app.selected, Some(0));
        app.handle_key(make_key(KeyCode::Char('k')));
        assert_eq!(app.selected, Some(0)); // stays at 0
    }

    #[test]
    fn key_j_stops_at_end() {
        let mut app = test_app();
        let count = app.sorted_disks().len();
        app.selected = Some(count - 1);
        app.handle_key(make_key(KeyCode::Char('j')));
        assert_eq!(app.selected, Some(count - 1));
    }

    #[test]
    fn key_upper_g_jumps_to_last() {
        let mut app = test_app();
        let count = app.sorted_disks().len();
        app.handle_key(make_key(KeyCode::Char('G')));
        assert_eq!(app.selected, Some(count - 1));
    }

    #[test]
    fn key_home_jumps_to_first() {
        let mut app = test_app();
        app.selected = Some(3);
        app.handle_key(make_key(KeyCode::Home));
        assert_eq!(app.selected, Some(0));
    }

    #[test]
    fn key_end_jumps_to_last() {
        let mut app = test_app();
        let count = app.sorted_disks().len();
        app.handle_key(make_key(KeyCode::End));
        assert_eq!(app.selected, Some(count - 1));
    }

    #[test]
    fn key_esc_deselects() {
        let mut app = test_app();
        app.selected = Some(2);
        app.handle_key(make_key(KeyCode::Esc));
        assert_eq!(app.selected, None);
    }

    #[test]
    fn ctrl_d_half_page_down() {
        let mut app = test_app();
        app.selected = Some(0);
        let count = app.sorted_disks().len();
        app.handle_key(make_ctrl_key(KeyCode::Char('d')));
        assert_eq!(app.selected, Some((count / 2).min(count - 1)));
    }

    #[test]
    fn ctrl_u_half_page_up() {
        let mut app = test_app();
        let count = app.sorted_disks().len();
        app.selected = Some(count - 1);
        app.handle_key(make_ctrl_key(KeyCode::Char('u')));
        let expected = (count - 1).saturating_sub(count / 2);
        assert_eq!(app.selected, Some(expected));
    }

    #[test]
    fn ctrl_g_jumps_to_first() {
        let mut app = test_app();
        app.selected = Some(3);
        app.handle_key(make_ctrl_key(KeyCode::Char('g')));
        assert_eq!(app.selected, Some(0));
    }

    // ── Key handling — filter mode ─────────────────────────

    #[test]
    fn slash_enters_filter_mode() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        assert!(app.filter.active);
    }

    #[test]
    fn filter_mode_typing() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('h')));
        app.handle_key(make_key(KeyCode::Char('o')));
        app.handle_key(make_key(KeyCode::Char('m')));
        assert_eq!(app.filter.buf, "hom");
        // Live filter should be applied
        assert_eq!(app.filter.text, "hom");
    }

    #[test]
    fn filter_mode_enter_closes() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Enter));
        assert!(!app.filter.active);
        assert_eq!(app.filter.text, "a");
    }

    #[test]
    fn filter_mode_esc_restores_previous() {
        let mut app = test_app();
        app.filter.text = "old".into();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('x')));
        app.handle_key(make_key(KeyCode::Esc));
        assert!(!app.filter.active);
        assert_eq!(app.filter.text, "old");
    }

    #[test]
    fn filter_mode_backspace() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(app.filter.buf, "a");
        assert_eq!(app.filter.cursor, 1);
    }

    #[test]
    fn filter_mode_ctrl_a_moves_to_start() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        assert_eq!(app.filter.cursor, 2);
        app.handle_key(make_ctrl_key(KeyCode::Char('a')));
        assert_eq!(app.filter.cursor, 0);
    }

    #[test]
    fn filter_mode_ctrl_e_moves_to_end() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_ctrl_key(KeyCode::Char('a'))); // go to start
        app.handle_key(make_ctrl_key(KeyCode::Char('e'))); // go to end
        assert_eq!(app.filter.cursor, 2);
    }

    #[test]
    fn filter_mode_ctrl_u_clears_before_cursor() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Char('c')));
        // cursor at 3, clear before
        app.handle_key(make_ctrl_key(KeyCode::Char('u')));
        assert_eq!(app.filter.buf, "");
        assert_eq!(app.filter.cursor, 0);
    }

    #[test]
    fn filter_mode_ctrl_k_kills_to_end() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Char('c')));
        app.handle_key(make_ctrl_key(KeyCode::Char('a'))); // go to start
        app.handle_key(make_key(KeyCode::Right)); // move to pos 1
        app.handle_key(make_ctrl_key(KeyCode::Char('k'))); // kill from pos 1
        assert_eq!(app.filter.buf, "a");
    }

    #[test]
    fn key_0_clears_filter() {
        let mut app = test_app();
        app.filter.text = "test".into();
        app.filter.buf = "test".into();
        app.handle_key(make_key(KeyCode::Char('0')));
        assert!(app.filter.text.is_empty());
        assert!(app.filter.buf.is_empty());
    }

    // ── Key handling — filter swallows in help mode ────────

    #[test]
    fn keys_swallowed_in_help_mode() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Char('b'))); // should not cycle bar style
        assert_eq!(app.prefs.bar_style, BarStyle::Gradient); // unchanged
        assert!(app.show_help); // 'b' does not dismiss help
        // Only q/h/esc/j/k dismiss help
        app.handle_key(make_key(KeyCode::Esc));
        assert!(!app.show_help);
    }

    // ── Key handling — ctrl swallows unknown combos ────────

    #[test]
    fn ctrl_unknown_swallowed() {
        let mut app = test_app();
        let prev_sort = app.prefs.sort_mode;
        app.handle_key(make_ctrl_key(KeyCode::Char('z')));
        assert_eq!(app.prefs.sort_mode, prev_sort); // unchanged
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

    // ── Navigation on empty disk list ─────────────────────

    #[test]
    fn navigation_empty_disks_j_noop() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.handle_key(make_key(KeyCode::Char('j')));
        assert_eq!(app.selected, None);
    }

    #[test]
    fn navigation_empty_disks_k_noop() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.handle_key(make_key(KeyCode::Char('k')));
        assert_eq!(app.selected, None);
    }

    #[test]
    fn navigation_empty_disks_g_noop() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.handle_key(make_key(KeyCode::Char('G')));
        assert_eq!(app.selected, None);
    }

    #[test]
    fn navigation_empty_disks_home_noop() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.handle_key(make_key(KeyCode::Home));
        assert_eq!(app.selected, None);
    }

    #[test]
    fn navigation_empty_disks_end_noop() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.handle_key(make_key(KeyCode::End));
        assert_eq!(app.selected, None);
    }

    #[test]
    fn navigation_empty_disks_ctrl_d_noop() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.handle_key(make_ctrl_key(KeyCode::Char('d')));
        assert_eq!(app.selected, None);
    }

    #[test]
    fn navigation_empty_disks_ctrl_u_noop() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.handle_key(make_ctrl_key(KeyCode::Char('u')));
        assert_eq!(app.selected, None);
    }

    #[test]
    fn navigation_empty_disks_ctrl_g_noop() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.handle_key(make_ctrl_key(KeyCode::Char('g')));
        assert_eq!(app.selected, None);
    }

    // ── Down/Up arrow keys ────────────────────────────────

    #[test]
    fn down_arrow_selects_next() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Down));
        assert_eq!(app.selected, Some(0));
        app.handle_key(make_key(KeyCode::Down));
        assert_eq!(app.selected, Some(1));
    }

    #[test]
    fn up_arrow_selects_prev() {
        let mut app = test_app();
        app.selected = Some(2);
        app.handle_key(make_key(KeyCode::Up));
        assert_eq!(app.selected, Some(1));
    }

    // ── Filter mode — Delete key ──────────────────────────

    #[test]
    fn filter_mode_delete_at_cursor() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Char('c')));
        // Move cursor to position 1
        app.handle_key(make_ctrl_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Right));
        assert_eq!(app.filter.cursor, 1);
        // Delete at cursor removes 'b'
        app.handle_key(make_key(KeyCode::Delete));
        assert_eq!(app.filter.buf, "ac");
        assert_eq!(app.filter.cursor, 1);
    }

    #[test]
    fn filter_mode_delete_at_end_noop() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Delete));
        assert_eq!(app.filter.buf, "a");
    }

    // ── Filter mode — Ctrl+w word delete ──────────────────

    #[test]
    fn filter_mode_ctrl_w_deletes_word() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        for c in "hello world".chars() {
            app.handle_key(make_key(KeyCode::Char(c)));
        }
        assert_eq!(app.filter.buf, "hello world");
        app.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(app.filter.buf, "hello ");
        app.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(app.filter.buf, "");
    }

    #[test]
    fn filter_mode_ctrl_w_at_start_noop() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(app.filter.buf, "");
        assert_eq!(app.filter.cursor, 0);
    }

    // ── Filter mode — Ctrl+h backspace ────────────────────

    #[test]
    fn filter_mode_ctrl_h_backspace() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_ctrl_key(KeyCode::Char('h')));
        assert_eq!(app.filter.buf, "a");
        assert_eq!(app.filter.cursor, 1);
    }

    #[test]
    fn filter_mode_ctrl_h_at_start_noop() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_ctrl_key(KeyCode::Char('h')));
        assert_eq!(app.filter.buf, "");
        assert_eq!(app.filter.cursor, 0);
    }

    // ── Filter mode — Ctrl+b/f cursor movement ───────────

    #[test]
    fn filter_mode_ctrl_b_moves_left() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        assert_eq!(app.filter.cursor, 2);
        app.handle_key(make_ctrl_key(KeyCode::Char('b')));
        assert_eq!(app.filter.cursor, 1);
    }

    #[test]
    fn filter_mode_ctrl_b_at_start_stays() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_ctrl_key(KeyCode::Char('b')));
        assert_eq!(app.filter.cursor, 0);
    }

    #[test]
    fn filter_mode_ctrl_f_moves_right() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_ctrl_key(KeyCode::Char('a'))); // go to start
        app.handle_key(make_ctrl_key(KeyCode::Char('f')));
        assert_eq!(app.filter.cursor, 1);
    }

    #[test]
    fn filter_mode_ctrl_f_at_end_stays() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_ctrl_key(KeyCode::Char('f')));
        assert_eq!(app.filter.cursor, 1); // stays at end
    }

    // ── Filter mode — Left/Right arrows ───────────────────

    #[test]
    fn filter_mode_left_moves_cursor() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Left));
        assert_eq!(app.filter.cursor, 1);
    }

    #[test]
    fn filter_mode_right_moves_cursor() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_ctrl_key(KeyCode::Char('a'))); // start
        app.handle_key(make_key(KeyCode::Right));
        assert_eq!(app.filter.cursor, 1);
    }

    // ── Filter mode — Home/End ────────────────────────────

    #[test]
    fn filter_mode_home_moves_to_start() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Home));
        assert_eq!(app.filter.cursor, 0);
    }

    #[test]
    fn filter_mode_end_moves_to_end() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Home));
        app.handle_key(make_key(KeyCode::End));
        assert_eq!(app.filter.cursor, 2);
    }

    // ── Filter mode — insert at middle ────────────────────

    #[test]
    fn filter_mode_insert_at_middle() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('c')));
        app.handle_key(make_key(KeyCode::Left)); // cursor at 1
        app.handle_key(make_key(KeyCode::Char('b')));
        assert_eq!(app.filter.buf, "abc");
        assert_eq!(app.filter.cursor, 2);
    }

    // ── Filter mode — backspace at start noop ─────────────

    #[test]
    fn filter_mode_backspace_at_start_noop() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(app.filter.buf, "");
        assert_eq!(app.filter.cursor, 0);
    }

    // ── Filter — Ctrl+u at middle ─────────────────────────

    #[test]
    fn filter_mode_ctrl_u_at_middle() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        for c in "abcdef".chars() {
            app.handle_key(make_key(KeyCode::Char(c)));
        }
        // Move cursor to position 3
        app.handle_key(make_key(KeyCode::Home));
        app.handle_key(make_key(KeyCode::Right));
        app.handle_key(make_key(KeyCode::Right));
        app.handle_key(make_key(KeyCode::Right));
        assert_eq!(app.filter.cursor, 3);
        app.handle_key(make_ctrl_key(KeyCode::Char('u')));
        assert_eq!(app.filter.buf, "def");
        assert_eq!(app.filter.cursor, 0);
    }

    // ── Filter — unknown key in filter mode ───────────────

    #[test]
    fn filter_mode_unknown_key_ignored() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::F(1)));
        assert_eq!(app.filter.buf, "");
        assert!(app.filter.active);
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

    // ── Toggle show_local / show_all via keys ─────────────

    #[test]
    fn key_l_toggles_show_local() {
        let mut app = test_app();
        assert!(!app.prefs.show_local);
        app.handle_key(make_key(KeyCode::Char('l')));
        assert!(app.prefs.show_local);
        app.handle_key(make_key(KeyCode::Char('l')));
        assert!(!app.prefs.show_local);
    }

    #[test]
    fn key_a_toggles_show_all() {
        let mut app = test_app();
        assert!(app.prefs.show_all);
        app.handle_key(make_key(KeyCode::Char('a')));
        assert!(!app.prefs.show_all);
        app.handle_key(make_key(KeyCode::Char('a')));
        assert!(app.prefs.show_all);
    }

    // ── Help mode dismissal ───────────────────────────────

    #[test]
    fn help_dismisses_with_q() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Char('q')));
        assert!(!app.show_help);
        assert!(!app.quit); // q in help mode only dismisses, doesn't quit
    }

    #[test]
    fn help_dismisses_with_k() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Char('k')));
        assert!(!app.show_help);
    }

    #[test]
    fn help_dismisses_with_upper_q() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Char('Q')));
        assert!(!app.show_help);
    }

    #[test]
    fn help_dismisses_with_upper_h() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Char('H')));
        assert!(!app.show_help);
    }

    #[test]
    fn help_mode_swallows_all_other_keys() {
        let mut app = test_app();
        app.show_help = true;
        let prev = app.prefs.clone();
        // These should all be swallowed
        for c in ['s', 'n', 'u', 'r', 'c', 'i', 'v', 'd', 'g', 'x', 'w', 'm', 'f', 't', 'T', 'a', 'l', 'p', '/', '0', '?'] {
            app.handle_key(make_key(KeyCode::Char(c)));
        }
        // Help should still be shown (only q/h/esc/j/k dismiss)
        assert!(app.show_help);
        // Prefs should be unchanged
        assert_eq!(app.prefs.sort_mode, prev.sort_mode);
        assert_eq!(app.prefs.bar_style, prev.bar_style);
        assert_eq!(app.prefs.color_mode, prev.color_mode);
    }

    // ── Copy without selection ─────────────────────────────

    #[test]
    fn key_y_without_selection_sets_status() {
        let mut app = test_app();
        assert!(app.selected.is_none());
        app.handle_key(make_key(KeyCode::Char('y')));
        assert!(app.status_msg.is_some());
        assert!(app.status_msg.as_ref().unwrap().0.contains("Select a disk"));
    }

    // ── Export sets status message ─────────────────────────

    #[test]
    fn key_e_sets_status_msg() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('e')));
        assert!(app.status_msg.is_some());
        let msg = &app.status_msg.as_ref().unwrap().0;
        assert!(msg.contains("Export") || msg.contains("export"));
    }

    // ── Enter with no selection is noop ────────────────────

    #[test]
    fn key_enter_no_selection_noop() {
        let mut app = test_app();
        assert!(app.selected.is_none());
        app.handle_key(make_key(KeyCode::Enter));
        assert!(app.status_msg.is_none());
    }

    // ── Enter with out-of-bounds selection ─────────────────

    #[test]
    fn key_enter_oob_selection_noop() {
        let mut app = test_app();
        app.selected = Some(999);
        app.handle_key(make_key(KeyCode::Enter));
        // No crash, status may or may not be set depending on sorted_disks len
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

    // ── Ctrl+d from None ──────────────────────────────────

    #[test]
    fn ctrl_d_from_none() {
        let mut app = test_app();
        assert_eq!(app.selected, None);
        let count = app.sorted_disks().len();
        app.handle_key(make_ctrl_key(KeyCode::Char('d')));
        let jump = (count / 2).max(1);
        assert_eq!(app.selected, Some(jump.min(count - 1)));
    }

    #[test]
    fn ctrl_u_from_none() {
        let mut app = test_app();
        assert_eq!(app.selected, None);
        app.handle_key(make_ctrl_key(KeyCode::Char('u')));
        assert_eq!(app.selected, Some(0));
    }

    // ── Unknown key is noop ───────────────────────────────

    #[test]
    fn unknown_key_noop() {
        let mut app = test_app();
        let prev_mode = app.prefs.sort_mode;
        let prev_bars = app.prefs.show_bars;
        app.handle_key(make_key(KeyCode::F(12)));
        assert_eq!(app.prefs.sort_mode, prev_mode);
        assert_eq!(app.prefs.show_bars, prev_bars);
        assert!(!app.quit);
    }

    // ── Mount column width edge cases ─────────────────────

    #[test]
    fn mount_col_width_small_terminal() {
        let p = Prefs::default();
        let w = mount_col_width(30, &p);
        assert!(w >= 12); // max(30/3, 12) = 12
    }

    #[test]
    fn mount_col_width_custom_below_min() {
        let mut p = Prefs::default();
        p.col_mount_w = 3; // below min of 8
        assert_eq!(mount_col_width(120, &p), 8);
    }

    // ── Right col width dynamic edge cases ────────────────

    #[test]
    fn right_col_width_no_used() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.prefs.show_used = false;
        assert_eq!(right_col_width(&app), 7);
    }

    #[test]
    fn right_col_width_custom_override() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.prefs.col_bar_end_w = 40;
        assert_eq!(right_col_width(&app), 40);
    }

    #[test]
    fn right_col_width_custom_min_clamp() {
        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs = Prefs::default();
        app.prefs.col_bar_end_w = 2;
        assert_eq!(right_col_width(&app), 5); // min 5
    }

    // ── Mouse handling ────────────────────────────────────

    #[test]
    fn mouse_right_click_triggers_hover() {
        let mut app = test_app();
        assert!(app.hover.pos.is_none());
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 15, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert_eq!(app.hover.pos, Some((15, 5)));
        // Should be instantly ready (timestamp set in the past)
        assert!(app.hover_ready());
    }

    #[test]
    fn mouse_drag_mount_sep() {
        let mut app = test_app();
        let mount_w = mount_col_width(78, &app.prefs);
        let mount_sep_x = 1 + 3 + mount_w as u16;

        // Click near mount separator
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: mount_sep_x, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(matches!(app.drag, Some(DragTarget::MountSep)));

        // Drag to new position
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Drag(MouseButton::Left), column: 30, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.prefs.col_mount_w > 0);

        // Release
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), column: 30, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.drag.is_none());
    }

    #[test]
    fn mouse_up_without_drag_noop() {
        let mut app = test_app();
        assert!(app.drag.is_none());
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), column: 10, row: 10, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        // No crash, drag is still None
        assert!(app.drag.is_none());
    }

    #[test]
    fn mouse_scroll_and_other_events_noop() {
        let mut app = test_app();
        let prev_help = app.show_help;
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::ScrollUp, column: 10, row: 10, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert_eq!(app.show_help, prev_help);
    }

    // ── Mouse click to select disk ────────────────────────

    #[test]
    fn mouse_click_selects_disk_row() {
        let mut app = test_app();
        assert!(app.selected.is_none());
        // With border + header, first disk row is at y=5
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 10, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert_eq!(app.selected, Some(0));
    }

    #[test]
    fn mouse_click_selects_second_disk() {
        let mut app = test_app();
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 10, row: 6, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert_eq!(app.selected, Some(1));
    }

    #[test]
    fn mouse_click_out_of_range_no_select() {
        let mut app = test_app();
        // Click far below disk rows (row 50 is way past the 4 disks)
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 10, row: 50, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.selected.is_none());
    }

    #[test]
    fn mouse_click_already_selected_enters_drilldown() {
        let mut app = test_app();
        // First click selects
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 10, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert_eq!(app.selected, Some(0));
        assert_eq!(app.drill.mode, ViewMode::Disks);

        // Second click on same row enters drill-down
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 10, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert_eq!(app.drill.mode, ViewMode::DrillDown);
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

    // ── Slash in filter preserves prev filter ─────────────

    #[test]
    fn slash_preserves_previous_filter() {
        let mut app = test_app();
        app.filter.text = "old_filter".into();
        app.handle_key(make_key(KeyCode::Char('/')));
        assert!(app.filter.active);
        assert_eq!(app.filter.prev, "old_filter");
        assert_eq!(app.filter.buf, "old_filter");
        assert_eq!(app.filter.cursor, 10);
    }

    // ── Warn/crit threshold full cycles ───────────────────

    #[test]
    fn warn_threshold_full_cycle() {
        let mut app = test_app();
        app.prefs.thresh_warn = 50;
        app.handle_key(make_key(KeyCode::Char('t'))); assert_eq!(app.prefs.thresh_warn, 60);
        app.handle_key(make_key(KeyCode::Char('t'))); assert_eq!(app.prefs.thresh_warn, 70);
        app.handle_key(make_key(KeyCode::Char('t'))); assert_eq!(app.prefs.thresh_warn, 80);
        app.handle_key(make_key(KeyCode::Char('t'))); assert_eq!(app.prefs.thresh_warn, 50);
    }

    #[test]
    fn crit_threshold_full_cycle() {
        let mut app = test_app();
        app.prefs.thresh_crit = 80;
        app.handle_key(make_key(KeyCode::Char('T'))); assert_eq!(app.prefs.thresh_crit, 85);
        app.handle_key(make_key(KeyCode::Char('T'))); assert_eq!(app.prefs.thresh_crit, 90);
        app.handle_key(make_key(KeyCode::Char('T'))); assert_eq!(app.prefs.thresh_crit, 95);
        app.handle_key(make_key(KeyCode::Char('T'))); assert_eq!(app.prefs.thresh_crit, 80);
    }

    // ── Non-standard thresh value defaults to cycle start ─

    #[test]
    fn warn_threshold_nonstandard_resets() {
        let mut app = test_app();
        app.prefs.thresh_warn = 42; // non-standard
        app.handle_key(make_key(KeyCode::Char('t')));
        assert_eq!(app.prefs.thresh_warn, 50); // defaults to start
    }

    #[test]
    fn crit_threshold_nonstandard_resets() {
        let mut app = test_app();
        app.prefs.thresh_crit = 42;
        app.handle_key(make_key(KeyCode::Char('T')));
        assert_eq!(app.prefs.thresh_crit, 80);
    }

    // ── Refresh rate non-standard resets ───────────────────

    #[test]
    fn refresh_rate_nonstandard_resets() {
        let mut app = test_app();
        app.prefs.refresh_rate = 7;
        app.handle_key(make_key(KeyCode::Char('f')));
        assert_eq!(app.prefs.refresh_rate, 1);
    }

    // ── d key resets col_bar_end_w ────────────────────────

    #[test]
    fn key_d_resets_bar_end_width() {
        let mut app = test_app();
        app.prefs.col_bar_end_w = 30;
        app.handle_key(make_key(KeyCode::Char('d')));
        assert_eq!(app.prefs.col_bar_end_w, 0);
    }

    // ── m key resets col_mount_w ──────────────────────────

    #[test]
    fn key_m_resets_mount_width() {
        let mut app = test_app();
        app.prefs.col_mount_w = 25;
        app.handle_key(make_key(KeyCode::Char('m')));
        assert_eq!(app.prefs.col_mount_w, 0);
    }

    // ── Theme chooser live preview ────────────────────────

    #[test]
    fn theme_chooser_nav_auto_applies() {
        let mut app = test_app();
        assert_eq!(app.prefs.color_mode, ColorMode::Default);
        app.handle_key(make_key(KeyCode::Char('c'))); // open
        // Navigate down — should auto-apply
        app.handle_key(make_key(KeyCode::Char('j')));
        assert_eq!(app.prefs.color_mode, ColorMode::ALL[1]);
        // Navigate down again
        app.handle_key(make_key(KeyCode::Char('j')));
        assert_eq!(app.prefs.color_mode, ColorMode::ALL[2]);
    }

    #[test]
    fn theme_chooser_esc_reverts_to_original() {
        let mut app = test_app();
        assert_eq!(app.prefs.color_mode, ColorMode::Default);
        app.handle_key(make_key(KeyCode::Char('c')));
        // Navigate to change theme
        app.handle_key(make_key(KeyCode::Char('j')));
        app.handle_key(make_key(KeyCode::Char('j')));
        assert_ne!(app.prefs.color_mode, ColorMode::Default);
        // Esc should revert
        app.handle_key(make_key(KeyCode::Esc));
        assert!(!app.theme_chooser.active);
        assert_eq!(app.prefs.color_mode, ColorMode::Default);
    }

    #[test]
    fn theme_chooser_stores_original() {
        let mut app = test_app();
        app.prefs.color_mode = ColorMode::Purple;
        app.handle_key(make_key(KeyCode::Char('c')));
        assert_eq!(app.theme_chooser.orig_color_mode, ColorMode::Purple);
        assert!(app.theme_chooser.orig_active_theme.is_none());
    }

    #[test]
    fn theme_chooser_home_end_auto_apply() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('c')));
        // Jump to end
        app.handle_key(make_key(KeyCode::Char('G')));
        let last_idx = app.all_themes().len() - 1;
        assert_eq!(app.theme_chooser.selected, last_idx);
        // Theme should have changed from Default
        assert_ne!(app.prefs.color_mode, ColorMode::Default);
        // Jump to start
        app.handle_key(make_key(KeyCode::Char('g')));
        assert_eq!(app.theme_chooser.selected, 0);
        assert_eq!(app.prefs.color_mode, ColorMode::Default);
    }

    // ── Theme chooser mouse interaction ───────────────────

    #[test]
    fn theme_chooser_mouse_click_selects() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('c')));
        assert!(app.theme_chooser.active);
        // Popup is centered. For 80x24 term, box_w=50, x0=15, y0 depends on theme count
        // Content starts at y0+2. Click on content area row 0 → first theme
        let themes = app.all_themes();
        let box_h = (themes.len() as u16 + 4).min(20); // 24 - 4 = 20
        let y0 = (24u16.saturating_sub(box_h)) / 2;
        let content_y = y0 + 2;
        // Click second row in content
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 30, row: content_y + 1, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert_eq!(app.theme_chooser.selected, 1);
        assert!(app.theme_chooser.active); // Still open after single click
    }

    #[test]
    fn theme_chooser_mouse_click_outside_cancels() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('c')));
        assert!(app.theme_chooser.active);
        // Click far outside popup
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: 0, row: 0, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(!app.theme_chooser.active);
        // Should revert to original
        assert_eq!(app.prefs.color_mode, ColorMode::Default);
    }

    #[test]
    fn theme_chooser_mouse_scroll_navigates() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('c')));
        assert_eq!(app.theme_chooser.selected, 0);
        // Scroll down
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::ScrollDown, column: 40, row: 12, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert_eq!(app.theme_chooser.selected, 1);
        // Auto-applied
        assert_eq!(app.prefs.color_mode, ColorMode::ALL[1]);
        // Scroll up
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::ScrollUp, column: 40, row: 12, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert_eq!(app.theme_chooser.selected, 0);
        assert_eq!(app.prefs.color_mode, ColorMode::ALL[0]);
    }

    // ── Right-click tooltip flag ──────────────────────────────

    #[test]
    fn right_click_sets_hover_right_click_flag() {
        let mut app = test_app();
        assert!(!app.hover.right_click);
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 15, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.hover.right_click);
        assert_eq!(app.hover.pos, Some((15, 5)));
        assert!(app.hover_ready());
    }

    #[test]
    fn mouse_move_clears_right_click_flag() {
        let mut app = test_app();
        // Right-click first
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 15, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.hover.right_click);
        // Move mouse
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Moved, column: 20, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(!app.hover.right_click);
        assert_eq!(app.hover.pos, Some((20, 5)));
    }

    #[test]
    fn mouse_move_same_pos_keeps_right_click_flag() {
        let mut app = test_app();
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 15, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.hover.right_click);
        // Move to same position — should not clear
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Moved, column: 15, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.hover.right_click);
    }

    // ── Drag priority over header sort ────────────────────────

    #[test]
    fn drag_pct_sep_takes_priority_over_sort() {
        let mut app = test_app();
        app.prefs.show_used = true;
        let right_w = right_col_width(&app);
        let pct_w: u16 = if app.prefs.col_pct_w > 0 { app.prefs.col_pct_w } else { 5 };
        let right_start = 80u16.saturating_sub(1 + right_w);
        let pct_sep_x = right_start + pct_w;
        let header_row: u16 = 3; // show_border default = true, so header_row = 3

        // Click at pct separator on header row
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: pct_sep_x, row: header_row, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        // Should start drag, not sort
        assert!(matches!(app.drag, Some(DragTarget::PctSep)));
    }

    #[test]
    fn drag_bar_end_sep_takes_priority_over_sort() {
        let mut app = test_app();
        let right_w = right_col_width(&app);
        let bar_end_x = 80u16.saturating_sub(1 + right_w + 1);
        let header_row: u16 = 3;

        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: bar_end_x, row: header_row, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(matches!(app.drag, Some(DragTarget::BarEndSep)));
    }

    #[test]
    fn drag_pct_sep_and_release() {
        let mut app = test_app();
        app.prefs.show_used = true;
        let right_w = right_col_width(&app);
        let pct_w: u16 = if app.prefs.col_pct_w > 0 { app.prefs.col_pct_w } else { 5 };
        let right_start = 80u16.saturating_sub(1 + right_w);
        let pct_sep_x = right_start + pct_w;

        // Start drag
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: pct_sep_x, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(matches!(app.drag, Some(DragTarget::PctSep)));

        // Drag to new position
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Drag(MouseButton::Left), column: pct_sep_x + 3, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.prefs.col_pct_w > 0);

        // Release
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), column: pct_sep_x + 3, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.drag.is_none());
    }

    #[test]
    fn drag_bar_end_sep_and_release() {
        let mut app = test_app();
        let right_w = right_col_width(&app);
        let bar_end_x = 80u16.saturating_sub(1 + right_w + 1);

        // Start drag
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: bar_end_x, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(matches!(app.drag, Some(DragTarget::BarEndSep)));

        // Drag
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Drag(MouseButton::Left), column: bar_end_x - 5, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.prefs.col_bar_end_w > 0);

        // Release
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), column: bar_end_x - 5, row: 5, modifiers: KeyModifiers::NONE },
            80, 24,
        );
        assert!(app.drag.is_none());
    }
}
