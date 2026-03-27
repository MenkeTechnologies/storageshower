use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use sysinfo::DiskKind;

use crate::helpers::format_bytes;
use crate::prefs::{load_prefs, save_prefs, Prefs};
use crate::system::chrono_now;
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
}

impl App {
    pub fn new(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>) -> Self {
        let prefs = load_prefs();
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
        }
    }

    pub fn refresh_data(&mut self) {
        if self.paused {
            return;
        }
        let (stats, disks) = self.shared_stats.lock().unwrap().clone();
        self.stats = stats;
        self.disks = disks;
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
                }
                KeyCode::Esc => {
                    self.filter = self.filter_prev.clone();
                    self.filter_mode = false;
                    self.filter_cursor = 0;
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
                self.prefs.color_mode = match self.prefs.color_mode {
                    ColorMode::Default => ColorMode::Green,
                    ColorMode::Green => ColorMode::Blue,
                    ColorMode::Blue => ColorMode::Purple,
                    ColorMode::Purple => ColorMode::Default,
                };
                self.save();
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
