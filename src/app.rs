use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use sysinfo::DiskKind;

use crate::helpers::format_bytes;
use crate::cli::Cli;
use crate::prefs::{load_prefs_from, save_prefs, Prefs};
use crate::system::{chrono_now, scan_directory};
use crate::types::*;

pub struct App {
    pub prefs: Prefs,
    pub disks: Vec<DiskEntry>,
    pub stats: SysStats,
    pub shared_stats: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>,
    pub paused: bool,
    pub show_help: bool,
    pub filter: String,
    pub filter_mode: bool,
    pub filter_buf: String,
    pub filter_prev: String,
    pub filter_cursor: usize,
    pub quit: bool,
    pub drag: Option<DragTarget>,
    pub selected: Option<usize>,
    pub status_msg: Option<(String, Instant)>,
    // Theme editor state
    pub theme_editor: bool,
    pub theme_edit_colors: [u8; 6],
    pub theme_edit_slot: usize,
    pub theme_edit_naming: bool,
    pub theme_edit_name: String,
    pub theme_edit_cursor: usize,
    // Drill-down state
    pub view_mode: ViewMode,
    pub drill_path: Vec<String>,
    pub drill_entries: Vec<DirEntry>,
    pub drill_selected: usize,
    pub drill_scanning: bool,
    pub drill_scan_result: Arc<Mutex<Option<Vec<DirEntry>>>>,
}

impl App {
    pub fn new(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>, cli: &Cli) -> Self {
        let mut prefs = load_prefs_from(cli.config.as_deref());
        cli.apply_to(&mut prefs);
        let (stats, disks) = shared.lock().unwrap().clone();
        Self {
            prefs,
            disks,
            stats,
            shared_stats: shared,
            paused: false,
            show_help: false,
            filter: String::new(),
            filter_mode: false,
            filter_buf: String::new(),
            filter_prev: String::new(),
            filter_cursor: 0,
            quit: false,
            selected: None,
            status_msg: None,
            drag: None,
            theme_editor: false,
            theme_edit_colors: [0; 6],
            theme_edit_slot: 0,
            theme_edit_naming: false,
            theme_edit_name: String::new(),
            theme_edit_cursor: 0,
            view_mode: ViewMode::Disks,
            drill_path: Vec::new(),
            drill_entries: Vec::new(),
            drill_selected: 0,
            drill_scanning: false,
            drill_scan_result: Arc::new(Mutex::new(None)),
        }
    }

    /// Create with default prefs (no CLI overrides).
    pub fn new_default(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>) -> Self {
        let prefs = Prefs::default();
        let (stats, disks) = shared.lock().unwrap().clone();
        Self {
            prefs,
            disks,
            stats,
            shared_stats: shared,
            paused: false,
            show_help: false,
            filter: String::new(),
            filter_mode: false,
            filter_buf: String::new(),
            filter_prev: String::new(),
            filter_cursor: 0,
            quit: false,
            selected: None,
            status_msg: None,
            drag: None,
            theme_editor: false,
            theme_edit_colors: [0; 6],
            theme_edit_slot: 0,
            theme_edit_naming: false,
            theme_edit_name: String::new(),
            theme_edit_cursor: 0,
            view_mode: ViewMode::Disks,
            drill_path: Vec::new(),
            drill_entries: Vec::new(),
            drill_selected: 0,
            drill_scanning: false,
            drill_scan_result: Arc::new(Mutex::new(None)),
        }
    }

    pub fn refresh_data(&mut self) {
        // Check for completed drill-down scans
        if self.drill_scanning {
            let mut result = self.drill_scan_result.lock().unwrap();
            if let Some(entries) = result.take() {
                self.drill_entries = entries;
                self.drill_scanning = false;
                self.drill_selected = 0;
            }
        }

        if self.paused {
            return;
        }
        let (stats, disks) = self.shared_stats.lock().unwrap().clone();
        self.stats = stats;
        self.disks = disks;
    }

    fn start_drill_scan(&mut self, path: &str) {
        self.drill_scanning = true;
        self.drill_entries.clear();
        let result = Arc::clone(&self.drill_scan_result);
        let path = path.to_string();
        std::thread::spawn(move || {
            let entries = scan_directory(&path);
            let mut lock = result.lock().unwrap();
            *lock = Some(entries);
        });
    }

    pub fn drill_current_path(&self) -> String {
        self.drill_path.last().cloned().unwrap_or_default()
    }

    pub fn sorted_disks(&self) -> Vec<DiskEntry> {
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
        if !self.filter.is_empty() {
            let f = self.filter.to_lowercase();
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
        ds
    }

    pub fn save(&self) {
        save_prefs(&self.prefs);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        if self.filter_mode {
            match key.code {
                KeyCode::Enter => {
                    self.filter_mode = false;
                    return;
                }
                KeyCode::Esc => {
                    self.filter = self.filter_prev.clone();
                    self.filter_mode = false;
                    self.filter_cursor = 0;
                    return;
                }
                KeyCode::Backspace => {
                    if self.filter_cursor > 0 {
                        self.filter_cursor -= 1;
                        self.filter_buf.remove(self.filter_cursor);
                    }
                }
                KeyCode::Delete => {
                    if self.filter_cursor < self.filter_buf.len() {
                        self.filter_buf.remove(self.filter_cursor);
                    }
                }
                KeyCode::Char('w') if ctrl => {
                    if self.filter_cursor > 0 {
                        let before = &self.filter_buf[..self.filter_cursor];
                        let trimmed = before.trim_end();
                        let word_start = trimmed.rfind(' ').map(|i| i + 1).unwrap_or(0);
                        self.filter_buf.drain(word_start..self.filter_cursor);
                        self.filter_cursor = word_start;
                    }
                }
                KeyCode::Char('u') if ctrl => {
                    self.filter_buf.drain(..self.filter_cursor);
                    self.filter_cursor = 0;
                }
                KeyCode::Char('k') if ctrl => {
                    self.filter_buf.truncate(self.filter_cursor);
                }
                KeyCode::Char('a') if ctrl => {
                    self.filter_cursor = 0;
                }
                KeyCode::Home => {
                    self.filter_cursor = 0;
                }
                KeyCode::Char('e') if ctrl => {
                    self.filter_cursor = self.filter_buf.len();
                }
                KeyCode::End => {
                    self.filter_cursor = self.filter_buf.len();
                }
                KeyCode::Char('b') if ctrl => {
                    self.filter_cursor = self.filter_cursor.saturating_sub(1);
                }
                KeyCode::Left => {
                    self.filter_cursor = self.filter_cursor.saturating_sub(1);
                }
                KeyCode::Char('f') if ctrl => {
                    self.filter_cursor = (self.filter_cursor + 1).min(self.filter_buf.len());
                }
                KeyCode::Right => {
                    self.filter_cursor = (self.filter_cursor + 1).min(self.filter_buf.len());
                }
                KeyCode::Char('h') if ctrl => {
                    if self.filter_cursor > 0 {
                        self.filter_cursor -= 1;
                        self.filter_buf.remove(self.filter_cursor);
                    }
                }
                KeyCode::Char(c) => {
                    self.filter_buf.insert(self.filter_cursor, c);
                    self.filter_cursor += 1;
                }
                _ => {}
            }
            self.filter = self.filter_buf.clone();
            return;
        }

        if self.show_help {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Esc
                | KeyCode::Char('j') | KeyCode::Char('k') => {
                    self.show_help = false;
                }
                _ => {}
            }
            return;
        }

        if self.theme_editor {
            if self.theme_edit_naming {
                match key.code {
                    KeyCode::Enter => {
                        let name = self.theme_edit_name.trim().to_string();
                        if !name.is_empty() {
                            let colors = self.theme_edit_colors;
                            self.prefs.custom_themes.insert(name.clone(), ThemeColors {
                                blue: colors[0],
                                green: colors[1],
                                purple: colors[2],
                                light_purple: colors[3],
                                royal: colors[4],
                                dark_purple: colors[5],
                            });
                            self.prefs.active_theme = Some(name.clone());
                            self.save();
                            self.status_msg = Some((format!("Saved theme: {}", name), Instant::now()));
                        }
                        self.theme_editor = false;
                        self.theme_edit_naming = false;
                        self.theme_edit_name.clear();
                        self.theme_edit_cursor = 0;
                    }
                    KeyCode::Esc => {
                        self.theme_edit_naming = false;
                        self.theme_edit_name.clear();
                        self.theme_edit_cursor = 0;
                    }
                    KeyCode::Backspace => {
                        if self.theme_edit_cursor > 0 {
                            self.theme_edit_cursor -= 1;
                            self.theme_edit_name.remove(self.theme_edit_cursor);
                        }
                    }
                    KeyCode::Left => {
                        self.theme_edit_cursor = self.theme_edit_cursor.saturating_sub(1);
                    }
                    KeyCode::Right => {
                        self.theme_edit_cursor = (self.theme_edit_cursor + 1).min(self.theme_edit_name.len());
                    }
                    KeyCode::Char(c) if !ctrl => {
                        if self.theme_edit_name.len() < 20 {
                            self.theme_edit_name.insert(self.theme_edit_cursor, c);
                            self.theme_edit_cursor += 1;
                        }
                    }
                    _ => {}
                }
                return;
            }

            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.theme_editor = false;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    self.theme_edit_slot = (self.theme_edit_slot + 1).min(5);
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.theme_edit_slot = self.theme_edit_slot.saturating_sub(1);
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    self.theme_edit_colors[self.theme_edit_slot] =
                        self.theme_edit_colors[self.theme_edit_slot].wrapping_add(1);
                }
                KeyCode::Char('h') | KeyCode::Left => {
                    self.theme_edit_colors[self.theme_edit_slot] =
                        self.theme_edit_colors[self.theme_edit_slot].wrapping_sub(1);
                }
                KeyCode::Char('L') => {
                    self.theme_edit_colors[self.theme_edit_slot] =
                        self.theme_edit_colors[self.theme_edit_slot].wrapping_add(10);
                }
                KeyCode::Char('H') => {
                    self.theme_edit_colors[self.theme_edit_slot] =
                        self.theme_edit_colors[self.theme_edit_slot].wrapping_sub(10);
                }
                KeyCode::Enter | KeyCode::Char('s') | KeyCode::Char('S') => {
                    self.theme_edit_naming = true;
                    self.theme_edit_name.clear();
                    self.theme_edit_cursor = 0;
                }
                _ => {}
            }
            return;
        }

        if self.view_mode == ViewMode::DrillDown {
            match key.code {
                KeyCode::Esc | KeyCode::Backspace => {
                    if self.drill_path.len() > 1 {
                        self.drill_path.pop();
                        let parent = self.drill_current_path();
                        self.start_drill_scan(&parent);
                    } else {
                        self.view_mode = ViewMode::Disks;
                        self.drill_path.clear();
                        self.drill_entries.clear();
                    }
                }
                KeyCode::Enter => {
                    if !self.drill_scanning {
                        if let Some(entry) = self.drill_entries.get(self.drill_selected) {
                            if entry.is_dir {
                                let path = entry.path.clone();
                                self.drill_path.push(path.clone());
                                self.start_drill_scan(&path);
                            }
                        }
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if !self.drill_entries.is_empty() {
                        self.drill_selected = (self.drill_selected + 1).min(self.drill_entries.len() - 1);
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.drill_selected = self.drill_selected.saturating_sub(1);
                }
                KeyCode::Home | KeyCode::Char('g') => {
                    self.drill_selected = 0;
                }
                KeyCode::End | KeyCode::Char('G') => {
                    if !self.drill_entries.is_empty() {
                        self.drill_selected = self.drill_entries.len() - 1;
                    }
                }
                KeyCode::Char('o') | KeyCode::Char('O') => {
                    let path = self.drill_current_path();
                    #[cfg(target_os = "macos")]
                    { let _ = std::process::Command::new("open").arg(&path).spawn(); }
                    #[cfg(target_os = "linux")]
                    { let _ = std::process::Command::new("xdg-open").arg(&path).spawn(); }
                    self.status_msg = Some((format!("Opened {}", path), Instant::now()));
                }
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.quit = true;
                }
                _ => {}
            }
            return;
        }

        if ctrl {
            match key.code {
                KeyCode::Char('d') => {
                    let count = self.sorted_disks().len();
                    if count > 0 {
                        let jump = (count / 2).max(1);
                        self.selected = Some(match self.selected {
                            Some(i) => (i + jump).min(count - 1),
                            None => jump.min(count - 1),
                        });
                    }
                    return;
                }
                KeyCode::Char('u') => {
                    let count = self.sorted_disks().len();
                    if count > 0 {
                        let jump = (count / 2).max(1);
                        self.selected = Some(match self.selected {
                            Some(i) => i.saturating_sub(jump),
                            None => 0,
                        });
                    }
                    return;
                }
                KeyCode::Char('g') => {
                    let count = self.sorted_disks().len();
                    if count > 0 {
                        self.selected = Some(0);
                    }
                    return;
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Esc => {
                self.selected = None;
            }
            KeyCode::Home => {
                let count = self.sorted_disks().len();
                if count > 0 {
                    self.selected = Some(0);
                }
            }
            KeyCode::End => {
                let count = self.sorted_disks().len();
                if count > 0 {
                    self.selected = Some(count - 1);
                }
            }
            KeyCode::Char('G') => {
                let count = self.sorted_disks().len();
                if count > 0 {
                    self.selected = Some(count - 1);
                }
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.quit = true;
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.show_help = true;
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                self.paused = !self.paused;
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                self.prefs.show_local = !self.prefs.show_local;
                self.save();
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.prefs.show_all = !self.prefs.show_all;
                self.save();
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.prefs.sort_rev = !self.prefs.sort_rev;
                self.save();
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                if self.prefs.sort_mode == SortMode::Name {
                    self.prefs.sort_rev = !self.prefs.sort_rev;
                } else {
                    self.prefs.sort_mode = SortMode::Name;
                    self.prefs.sort_rev = false;
                }
                self.save();
            }
            KeyCode::Char('u') | KeyCode::Char('U') => {
                if self.prefs.sort_mode == SortMode::Pct {
                    self.prefs.sort_rev = !self.prefs.sort_rev;
                } else {
                    self.prefs.sort_mode = SortMode::Pct;
                    self.prefs.sort_rev = false;
                }
                self.save();
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                if self.prefs.sort_mode == SortMode::Size {
                    self.prefs.sort_rev = !self.prefs.sort_rev;
                } else {
                    self.prefs.sort_mode = SortMode::Size;
                    self.prefs.sort_rev = false;
                }
                self.save();
            }
            KeyCode::Char('b') => {
                self.prefs.bar_style = match self.prefs.bar_style {
                    BarStyle::Gradient => BarStyle::Solid,
                    BarStyle::Solid => BarStyle::Thin,
                    BarStyle::Thin => BarStyle::Ascii,
                    BarStyle::Ascii => BarStyle::Gradient,
                };
                self.save();
            }
            KeyCode::Char('c') => {
                let custom_names: Vec<String> = {
                    let mut names: Vec<String> = self.prefs.custom_themes.keys().cloned().collect();
                    names.sort();
                    names
                };
                if let Some(ref active) = self.prefs.active_theme {
                    // Currently on a custom theme — find next custom or wrap to first built-in
                    if let Some(pos) = custom_names.iter().position(|n| n == active) {
                        if pos + 1 < custom_names.len() {
                            self.prefs.active_theme = Some(custom_names[pos + 1].clone());
                        } else {
                            self.prefs.active_theme = None;
                            self.prefs.color_mode = ColorMode::ALL[0];
                        }
                    } else {
                        self.prefs.active_theme = None;
                        self.prefs.color_mode = ColorMode::ALL[0];
                    }
                } else {
                    // Currently on a built-in theme
                    let next = self.prefs.color_mode.next();
                    if next == ColorMode::ALL[0] && !custom_names.is_empty() {
                        // Wrapped around — enter custom themes
                        self.prefs.active_theme = Some(custom_names[0].clone());
                    } else {
                        self.prefs.color_mode = next;
                    }
                }
                let display_name = if let Some(ref name) = self.prefs.active_theme {
                    name.clone()
                } else {
                    self.prefs.color_mode.name().to_string()
                };
                self.status_msg = Some((
                    format!("\u{25C6} {}", display_name),
                    Instant::now(),
                ));
                self.save();
            }
            KeyCode::Char('C') => {
                // Open theme editor, seeded with current palette
                let current = crate::ui::palette_for_prefs(&self.prefs);
                fn idx(c: ratatui::style::Color) -> u8 {
                    match c {
                        ratatui::style::Color::Indexed(n) => n,
                        _ => 0,
                    }
                }
                self.theme_edit_colors = [
                    idx(current.0), idx(current.1), idx(current.2),
                    idx(current.3), idx(current.4), idx(current.5),
                ];
                self.theme_edit_slot = 0;
                self.theme_editor = true;
                self.theme_edit_naming = false;
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                self.prefs.show_bars = !self.prefs.show_bars;
                self.save();
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.prefs.show_used = !self.prefs.show_used;
                self.prefs.col_bar_end_w = 0;
                self.save();
            }
            KeyCode::Char('g') => {
                self.prefs.show_header = !self.prefs.show_header;
                self.save();
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                self.prefs.show_border = !self.prefs.show_border;
                self.save();
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                self.prefs.compact = !self.prefs.compact;
                self.prefs.col_mount_w = 0;
                self.save();
            }
            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.prefs.full_mount = !self.prefs.full_mount;
                self.save();
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.prefs.unit_mode = match self.prefs.unit_mode {
                    UnitMode::Human => UnitMode::GiB,
                    UnitMode::GiB => UnitMode::MiB,
                    UnitMode::MiB => UnitMode::Bytes,
                    UnitMode::Bytes => UnitMode::Human,
                };
                self.save();
            }
            KeyCode::Char('t') => {
                self.prefs.thresh_warn = match self.prefs.thresh_warn {
                    50 => 60,
                    60 => 70,
                    70 => 80,
                    _ => 50,
                };
                self.save();
            }
            KeyCode::Char('T') => {
                self.prefs.thresh_crit = match self.prefs.thresh_crit {
                    80 => 85,
                    85 => 90,
                    90 => 95,
                    _ => 80,
                };
                self.save();
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                self.prefs.refresh_rate = match self.prefs.refresh_rate {
                    1 => 2,
                    2 => 5,
                    5 => 10,
                    _ => 1,
                };
                self.save();
            }
            KeyCode::Char('/') => {
                self.filter_mode = true;
                self.filter_prev = self.filter.clone();
                self.filter_buf = self.filter.clone();
                self.filter_cursor = self.filter_buf.len();
            }
            KeyCode::Char('0') => {
                self.filter.clear();
                self.filter_buf.clear();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let count = self.sorted_disks().len();
                if count > 0 {
                    self.selected = Some(match self.selected {
                        Some(i) => (i + 1).min(count - 1),
                        None => 0,
                    });
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let count = self.sorted_disks().len();
                if count > 0 {
                    self.selected = Some(match self.selected {
                        Some(i) => i.saturating_sub(1),
                        None => 0,
                    });
                }
            }
            KeyCode::Enter => {
                if let Some(idx) = self.selected {
                    let disks = self.sorted_disks();
                    if let Some(disk) = disks.get(idx) {
                        let mount = disk.mount.clone();
                        self.view_mode = ViewMode::DrillDown;
                        self.drill_path = vec![mount.clone()];
                        self.drill_selected = 0;
                        self.start_drill_scan(&mount);
                    }
                }
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                if let Some(idx) = self.selected {
                    let disks = self.sorted_disks();
                    if let Some(disk) = disks.get(idx) {
                        let mount = disk.mount.clone();
                        #[cfg(target_os = "macos")]
                        { let _ = std::process::Command::new("open").arg(&mount).spawn(); }
                        #[cfg(target_os = "linux")]
                        { let _ = std::process::Command::new("xdg-open").arg(&mount).spawn(); }
                        self.status_msg = Some((format!("Opened {}", mount), Instant::now()));
                    }
                }
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                let disks = self.sorted_disks();
                let mut out = String::from("DISK MATRIX EXPORT\n");
                out.push_str(&format!("Host: {}  Date: {} {}\n\n", self.stats.hostname, chrono_now().0, chrono_now().1));
                out.push_str(&format!("{:<30} {:>5} {:>10} {:>10}\n", "MOUNT", "PCT", "USED", "TOTAL"));
                out.push_str(&format!("{}\n", "-".repeat(60)));
                for d in &disks {
                    out.push_str(&format!("{:<30} {:>4.0}% {:>10} {:>10}\n",
                        d.mount, d.pct,
                        format_bytes(d.used, self.prefs.unit_mode),
                        format_bytes(d.total, self.prefs.unit_mode),
                    ));
                }
                let path = dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".storageshower.export.txt");
                match std::fs::write(&path, &out) {
                    Ok(_) => self.status_msg = Some((format!("Exported to {}", path.display()), Instant::now())),
                    Err(e) => self.status_msg = Some((format!("Export failed: {}", e), Instant::now())),
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Some(idx) = self.selected {
                    let disks = self.sorted_disks();
                    if let Some(disk) = disks.get(idx) {
                        let mount = disk.mount.clone();
                        let copied = std::process::Command::new("pbcopy")
                            .stdin(std::process::Stdio::piped())
                            .spawn()
                            .and_then(|mut child| {
                                use std::io::Write;
                                if let Some(ref mut stdin) = child.stdin {
                                    stdin.write_all(mount.as_bytes())?;
                                }
                                child.wait()
                            });
                        match copied {
                            Ok(_) => self.status_msg = Some((format!("Copied: {}", mount), Instant::now())),
                            Err(_) => self.status_msg = Some(("Copy failed (pbcopy not found)".into(), Instant::now())),
                        }
                    }
                } else {
                    self.status_msg = Some(("Select a disk first (j/k)".into(), Instant::now()));
                }
            }
            KeyCode::Char('?') => {
                self.show_help = true;
            }
            _ => {}
        }
    }

    pub fn handle_mouse(&mut self, event: MouseEvent, term_w: u16) {
        let show_border = self.prefs.show_border;
        let lm: u16 = if show_border { 1 } else { 0 };
        let rm: u16 = if show_border { 1 } else { 0 };
        let inner_w = term_w.saturating_sub(lm + rm);

        let mount_w = mount_col_width(inner_w, &self.prefs);
        let mount_sep_x = lm + 3 + mount_w as u16;

        let right_w = right_col_width_static(&self.prefs);
        let bar_end_x = term_w.saturating_sub(rm + right_w + 1);
        let pct_w: u16 = if self.prefs.col_pct_w > 0 { self.prefs.col_pct_w } else { 5 };
        let right_start = term_w.saturating_sub(rm + right_w);
        let pct_sep_x = right_start + pct_w;

        let header_row: u16 = if show_border { 3 } else { 2 };

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let x = event.column;
                let y = event.row;

                if self.prefs.show_header && y == header_row {
                    let clicked_sort = if x >= lm && x < mount_sep_x {
                        Some(SortMode::Name)
                    } else if x > bar_end_x && x < right_start {
                        None
                    } else if x >= right_start && x < pct_sep_x {
                        Some(SortMode::Pct)
                    } else if self.prefs.show_used && x > pct_sep_x {
                        Some(SortMode::Size)
                    } else {
                        None
                    };

                    if let Some(mode) = clicked_sort {
                        if self.prefs.sort_mode == mode {
                            self.prefs.sort_rev = !self.prefs.sort_rev;
                        } else {
                            self.prefs.sort_mode = mode;
                            self.prefs.sort_rev = false;
                        }
                        self.save();
                        return;
                    }
                }

                if x.abs_diff(mount_sep_x) <= 1 {
                    self.drag = Some(DragTarget::MountSep);
                } else if self.prefs.show_used && x.abs_diff(pct_sep_x) <= 1 {
                    self.drag = Some(DragTarget::PctSep);
                } else if x.abs_diff(bar_end_x) <= 1 {
                    self.drag = Some(DragTarget::BarEndSep);
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if let Some(target) = self.drag {
                    match target {
                        DragTarget::MountSep => {
                            let new_mount_w = event.column.saturating_sub(lm + 3);
                            let max_w = (inner_w as usize).saturating_sub(20);
                            let clamped = (new_mount_w as usize).clamp(8, max_w);
                            self.prefs.col_mount_w = clamped as u16;
                        }
                        DragTarget::BarEndSep => {
                            let new_right_w = term_w.saturating_sub(rm + event.column + 1);
                            let clamped = new_right_w.clamp(5, inner_w / 2);
                            self.prefs.col_bar_end_w = clamped;
                        }
                        DragTarget::PctSep => {
                            let new_pct_w = event.column.saturating_sub(right_start);
                            let clamped = new_pct_w.clamp(4, right_w.saturating_sub(6));
                            self.prefs.col_pct_w = clamped;
                        }
                    }
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.drag.is_some() {
                    self.drag = None;
                    self.save();
                }
            }
            MouseEventKind::Down(MouseButton::Right) => {
                self.show_help = !self.show_help;
            }
            _ => {}
        }
    }
}

// ─── Column width helpers ──────────────────────────────────────────────────

pub fn right_col_width_static(prefs: &Prefs) -> u16 {
    if prefs.col_bar_end_w > 0 {
        return prefs.col_bar_end_w.max(5);
    }
    if prefs.show_used { 22 } else { 7 }
}

pub fn right_col_width(app: &App) -> u16 {
    if app.prefs.col_bar_end_w > 0 {
        return app.prefs.col_bar_end_w.max(5);
    }
    if !app.prefs.show_used {
        return 7;
    }
    let disks = app.sorted_disks();
    let mut mu = 4usize;
    let mut mt = 4usize;
    for d in &disks {
        mu = mu.max(format_bytes(d.used, app.prefs.unit_mode).len());
        mt = mt.max(format_bytes(d.total, app.prefs.unit_mode).len());
    }
    let pct_w = if app.prefs.col_pct_w > 0 { app.prefs.col_pct_w as usize } else { 5 };
    let needed = pct_w + 1 + 1 + mu + 1 + mt + 1;
    (needed as u16).max(22)
}

pub fn mount_col_width(inner_w: u16, prefs: &Prefs) -> usize {
    if prefs.col_mount_w > 0 {
        return (prefs.col_mount_w as usize).clamp(8, (inner_w as usize).saturating_sub(20));
    }
    if prefs.compact {
        16
    } else {
        (inner_w as usize / 3).max(12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
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
        // Reset prefs to defaults so tests are deterministic
        // (load_prefs may read user's config from disk)
        app.prefs = Prefs::default();
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
        let disks = app.sorted_disks();
        let names: Vec<&str> = disks.iter().map(|d| d.mount.as_str()).collect();
        assert_eq!(names, vec!["/", "/data", "/home", "/tmp"]);
    }

    #[test]
    fn sorted_disks_by_name_reversed() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Name;
        app.prefs.sort_rev = true;
        let disks = app.sorted_disks();
        let names: Vec<&str> = disks.iter().map(|d| d.mount.as_str()).collect();
        assert_eq!(names, vec!["/tmp", "/home", "/data", "/"]);
    }

    #[test]
    fn sorted_disks_by_pct() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Pct;
        app.prefs.sort_rev = false;
        let disks = app.sorted_disks();
        let pcts: Vec<f64> = disks.iter().map(|d| d.pct).collect();
        assert!(pcts.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn sorted_disks_by_size() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Size;
        app.prefs.sort_rev = false;
        let disks = app.sorted_disks();
        let sizes: Vec<u64> = disks.iter().map(|d| d.total).collect();
        assert!(sizes.windows(2).all(|w| w[0] <= w[1]));
    }

    // ── Filtering ──────────────────────────────────────────

    #[test]
    fn sorted_disks_filter() {
        let mut app = test_app();
        app.filter = "home".into();
        let disks = app.sorted_disks();
        assert_eq!(disks.len(), 1);
        assert_eq!(disks[0].mount, "/home");
    }

    #[test]
    fn sorted_disks_filter_case_insensitive() {
        let mut app = test_app();
        app.filter = "HOME".into();
        let disks = app.sorted_disks();
        assert_eq!(disks.len(), 1);
        assert_eq!(disks[0].mount, "/home");
    }

    #[test]
    fn sorted_disks_filter_no_match() {
        let mut app = test_app();
        app.filter = "nonexistent".into();
        let disks = app.sorted_disks();
        assert!(disks.is_empty());
    }

    #[test]
    fn sorted_disks_show_all_off_filters_tmpfs() {
        let mut app = test_app();
        app.prefs.show_all = false;
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
    fn key_c_cycles_color_mode() {
        let mut app = test_app();
        assert_eq!(app.prefs.color_mode, ColorMode::Default);
        // Cycle through all modes and back to Default
        for &expected in &ColorMode::ALL[1..] {
            app.handle_key(make_key(KeyCode::Char('c')));
            assert_eq!(app.prefs.color_mode, expected);
            assert!(app.status_msg.is_some());
            assert!(app.status_msg.as_ref().unwrap().0.contains(expected.name()));
        }
        app.handle_key(make_key(KeyCode::Char('c')));
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
        assert!(app.filter_mode);
    }

    #[test]
    fn filter_mode_typing() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('h')));
        app.handle_key(make_key(KeyCode::Char('o')));
        app.handle_key(make_key(KeyCode::Char('m')));
        assert_eq!(app.filter_buf, "hom");
        // Live filter should be applied
        assert_eq!(app.filter, "hom");
    }

    #[test]
    fn filter_mode_enter_closes() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Enter));
        assert!(!app.filter_mode);
        assert_eq!(app.filter, "a");
    }

    #[test]
    fn filter_mode_esc_restores_previous() {
        let mut app = test_app();
        app.filter = "old".into();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('x')));
        app.handle_key(make_key(KeyCode::Esc));
        assert!(!app.filter_mode);
        assert_eq!(app.filter, "old");
    }

    #[test]
    fn filter_mode_backspace() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(app.filter_buf, "a");
        assert_eq!(app.filter_cursor, 1);
    }

    #[test]
    fn filter_mode_ctrl_a_moves_to_start() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        assert_eq!(app.filter_cursor, 2);
        app.handle_key(make_ctrl_key(KeyCode::Char('a')));
        assert_eq!(app.filter_cursor, 0);
    }

    #[test]
    fn filter_mode_ctrl_e_moves_to_end() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_ctrl_key(KeyCode::Char('a'))); // go to start
        app.handle_key(make_ctrl_key(KeyCode::Char('e'))); // go to end
        assert_eq!(app.filter_cursor, 2);
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
        assert_eq!(app.filter_buf, "");
        assert_eq!(app.filter_cursor, 0);
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
        assert_eq!(app.filter_buf, "a");
    }

    #[test]
    fn key_0_clears_filter() {
        let mut app = test_app();
        app.filter = "test".into();
        app.filter_buf = "test".into();
        app.handle_key(make_key(KeyCode::Char('0')));
        assert!(app.filter.is_empty());
        assert!(app.filter_buf.is_empty());
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
        assert_eq!(app.filter_cursor, 1);
        // Delete at cursor removes 'b'
        app.handle_key(make_key(KeyCode::Delete));
        assert_eq!(app.filter_buf, "ac");
        assert_eq!(app.filter_cursor, 1);
    }

    #[test]
    fn filter_mode_delete_at_end_noop() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Delete));
        assert_eq!(app.filter_buf, "a");
    }

    // ── Filter mode — Ctrl+w word delete ──────────────────

    #[test]
    fn filter_mode_ctrl_w_deletes_word() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        for c in "hello world".chars() {
            app.handle_key(make_key(KeyCode::Char(c)));
        }
        assert_eq!(app.filter_buf, "hello world");
        app.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(app.filter_buf, "hello ");
        app.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(app.filter_buf, "");
    }

    #[test]
    fn filter_mode_ctrl_w_at_start_noop() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(app.filter_buf, "");
        assert_eq!(app.filter_cursor, 0);
    }

    // ── Filter mode — Ctrl+h backspace ────────────────────

    #[test]
    fn filter_mode_ctrl_h_backspace() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_ctrl_key(KeyCode::Char('h')));
        assert_eq!(app.filter_buf, "a");
        assert_eq!(app.filter_cursor, 1);
    }

    #[test]
    fn filter_mode_ctrl_h_at_start_noop() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_ctrl_key(KeyCode::Char('h')));
        assert_eq!(app.filter_buf, "");
        assert_eq!(app.filter_cursor, 0);
    }

    // ── Filter mode — Ctrl+b/f cursor movement ───────────

    #[test]
    fn filter_mode_ctrl_b_moves_left() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        assert_eq!(app.filter_cursor, 2);
        app.handle_key(make_ctrl_key(KeyCode::Char('b')));
        assert_eq!(app.filter_cursor, 1);
    }

    #[test]
    fn filter_mode_ctrl_b_at_start_stays() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_ctrl_key(KeyCode::Char('b')));
        assert_eq!(app.filter_cursor, 0);
    }

    #[test]
    fn filter_mode_ctrl_f_moves_right() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_ctrl_key(KeyCode::Char('a'))); // go to start
        app.handle_key(make_ctrl_key(KeyCode::Char('f')));
        assert_eq!(app.filter_cursor, 1);
    }

    #[test]
    fn filter_mode_ctrl_f_at_end_stays() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_ctrl_key(KeyCode::Char('f')));
        assert_eq!(app.filter_cursor, 1); // stays at end
    }

    // ── Filter mode — Left/Right arrows ───────────────────

    #[test]
    fn filter_mode_left_moves_cursor() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Left));
        assert_eq!(app.filter_cursor, 1);
    }

    #[test]
    fn filter_mode_right_moves_cursor() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_ctrl_key(KeyCode::Char('a'))); // start
        app.handle_key(make_key(KeyCode::Right));
        assert_eq!(app.filter_cursor, 1);
    }

    // ── Filter mode — Home/End ────────────────────────────

    #[test]
    fn filter_mode_home_moves_to_start() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Home));
        assert_eq!(app.filter_cursor, 0);
    }

    #[test]
    fn filter_mode_end_moves_to_end() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Char('a')));
        app.handle_key(make_key(KeyCode::Char('b')));
        app.handle_key(make_key(KeyCode::Home));
        app.handle_key(make_key(KeyCode::End));
        assert_eq!(app.filter_cursor, 2);
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
        assert_eq!(app.filter_buf, "abc");
        assert_eq!(app.filter_cursor, 2);
    }

    // ── Filter mode — backspace at start noop ─────────────

    #[test]
    fn filter_mode_backspace_at_start_noop() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(app.filter_buf, "");
        assert_eq!(app.filter_cursor, 0);
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
        assert_eq!(app.filter_cursor, 3);
        app.handle_key(make_ctrl_key(KeyCode::Char('u')));
        assert_eq!(app.filter_buf, "def");
        assert_eq!(app.filter_cursor, 0);
    }

    // ── Filter — unknown key in filter mode ───────────────

    #[test]
    fn filter_mode_unknown_key_ignored() {
        let mut app = test_app();
        app.handle_key(make_key(KeyCode::Char('/')));
        app.handle_key(make_key(KeyCode::F(1)));
        assert_eq!(app.filter_buf, "");
        assert!(app.filter_mode);
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
        let disks = app.sorted_disks();
        let pcts: Vec<f64> = disks.iter().map(|d| d.pct).collect();
        assert!(pcts.windows(2).all(|w| w[0] >= w[1]));
    }

    #[test]
    fn sorted_disks_by_size_reversed() {
        let mut app = test_app();
        app.prefs.sort_mode = SortMode::Size;
        app.prefs.sort_rev = true;
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
        app.filter = "data".into();
        app.prefs.sort_mode = SortMode::Size;
        app.prefs.sort_rev = false;
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
    fn mouse_right_click_toggles_help() {
        let mut app = test_app();
        assert!(!app.show_help);
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 10, row: 10, modifiers: KeyModifiers::NONE },
            80,
        );
        assert!(app.show_help);
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: 10, row: 10, modifiers: KeyModifiers::NONE },
            80,
        );
        assert!(!app.show_help);
    }

    #[test]
    fn mouse_drag_mount_sep() {
        let mut app = test_app();
        let mount_w = mount_col_width(78, &app.prefs);
        let mount_sep_x = 1 + 3 + mount_w as u16;

        // Click near mount separator
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: mount_sep_x, row: 5, modifiers: KeyModifiers::NONE },
            80,
        );
        assert!(matches!(app.drag, Some(DragTarget::MountSep)));

        // Drag to new position
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Drag(MouseButton::Left), column: 30, row: 5, modifiers: KeyModifiers::NONE },
            80,
        );
        assert!(app.prefs.col_mount_w > 0);

        // Release
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), column: 30, row: 5, modifiers: KeyModifiers::NONE },
            80,
        );
        assert!(app.drag.is_none());
    }

    #[test]
    fn mouse_up_without_drag_noop() {
        let mut app = test_app();
        assert!(app.drag.is_none());
        app.handle_mouse(
            MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), column: 10, row: 10, modifiers: KeyModifiers::NONE },
            80,
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
            80,
        );
        assert_eq!(app.show_help, prev_help);
    }

    // ── Slash in filter preserves prev filter ─────────────

    #[test]
    fn slash_preserves_previous_filter() {
        let mut app = test_app();
        app.filter = "old_filter".into();
        app.handle_key(make_key(KeyCode::Char('/')));
        assert!(app.filter_mode);
        assert_eq!(app.filter_prev, "old_filter");
        assert_eq!(app.filter_buf, "old_filter");
        assert_eq!(app.filter_cursor, 10);
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
}
