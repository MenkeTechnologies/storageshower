use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use std::path::PathBuf;
use std::time::Instant;

use crate::app::{copy_to_clipboard, App};
use crate::columns::{mount_col_width, right_col_width};
use crate::helpers::format_bytes;
use crate::system::chrono_now;
use crate::types::*;

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        if self.filter.active {
            match key.code {
                KeyCode::Enter => {
                    self.filter.active = false;
                    return;
                }
                KeyCode::Esc => {
                    self.filter.text = self.filter.prev.clone();
                    self.filter.active = false;
                    self.filter.cursor = 0;
                    return;
                }
                KeyCode::Backspace => {
                    if self.filter.cursor > 0 {
                        self.filter.cursor -= 1;
                        self.filter.buf.remove(self.filter.cursor);
                    }
                }
                KeyCode::Delete => {
                    if self.filter.cursor < self.filter.buf.len() {
                        self.filter.buf.remove(self.filter.cursor);
                    }
                }
                KeyCode::Char('w') if ctrl => {
                    if self.filter.cursor > 0 {
                        let before = &self.filter.buf[..self.filter.cursor];
                        let trimmed = before.trim_end();
                        let word_start = trimmed.rfind(' ').map(|i| i + 1).unwrap_or(0);
                        self.filter.buf.drain(word_start..self.filter.cursor);
                        self.filter.cursor = word_start;
                    }
                }
                KeyCode::Char('u') if ctrl => {
                    self.filter.buf.drain(..self.filter.cursor);
                    self.filter.cursor = 0;
                }
                KeyCode::Char('k') if ctrl => {
                    self.filter.buf.truncate(self.filter.cursor);
                }
                KeyCode::Char('a') if ctrl => {
                    self.filter.cursor = 0;
                }
                KeyCode::Home => {
                    self.filter.cursor = 0;
                }
                KeyCode::Char('e') if ctrl => {
                    self.filter.cursor = self.filter.buf.len();
                }
                KeyCode::End => {
                    self.filter.cursor = self.filter.buf.len();
                }
                KeyCode::Char('b') if ctrl => {
                    self.filter.cursor = self.filter.cursor.saturating_sub(1);
                }
                KeyCode::Left => {
                    self.filter.cursor = self.filter.cursor.saturating_sub(1);
                }
                KeyCode::Char('f') if ctrl => {
                    self.filter.cursor = (self.filter.cursor + 1).min(self.filter.buf.len());
                }
                KeyCode::Right => {
                    self.filter.cursor = (self.filter.cursor + 1).min(self.filter.buf.len());
                }
                KeyCode::Char('h') if ctrl => {
                    if self.filter.cursor > 0 {
                        self.filter.cursor -= 1;
                        self.filter.buf.remove(self.filter.cursor);
                    }
                }
                KeyCode::Char(c) => {
                    self.filter.buf.insert(self.filter.cursor, c);
                    self.filter.cursor += 1;
                }
                _ => {}
            }
            self.filter.text = self.filter.buf.clone();
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

        if self.theme_chooser.active {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    // Revert to original theme
                    self.prefs.color_mode = self.theme_chooser.orig_color_mode;
                    self.prefs.active_theme = self.theme_chooser.orig_active_theme.clone();
                    self.theme_chooser.active = false;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    let count = self.all_themes().len();
                    if count > 0 {
                        self.theme_chooser.selected = (self.theme_chooser.selected + 1).min(count - 1);
                    }
                    self.apply_selected_theme();
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.theme_chooser.selected = self.theme_chooser.selected.saturating_sub(1);
                    self.apply_selected_theme();
                }
                KeyCode::Home | KeyCode::Char('g') => {
                    self.theme_chooser.selected = 0;
                    self.apply_selected_theme();
                }
                KeyCode::End | KeyCode::Char('G') => {
                    let count = self.all_themes().len();
                    if count > 0 {
                        self.theme_chooser.selected = count - 1;
                    }
                    self.apply_selected_theme();
                }
                KeyCode::Enter => {
                    self.apply_selected_theme();
                    let themes = self.all_themes();
                    if let Some((_, display)) = themes.get(self.theme_chooser.selected) {
                        self.status_msg = Some((format!("\u{25C6} {}", display), Instant::now()));
                    }
                    self.save();
                    self.theme_chooser.active = false;
                }
                _ => {}
            }
            return;
        }

        if self.theme_edit.active {
            if self.theme_edit.naming {
                match key.code {
                    KeyCode::Enter => {
                        let name = self.theme_edit.name.trim().to_string();
                        if !name.is_empty() {
                            let colors = self.theme_edit.colors;
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
                        self.theme_edit.active = false;
                        self.theme_edit.naming = false;
                        self.theme_edit.name.clear();
                        self.theme_edit.cursor = 0;
                    }
                    KeyCode::Esc => {
                        self.theme_edit.naming = false;
                        self.theme_edit.name.clear();
                        self.theme_edit.cursor = 0;
                    }
                    KeyCode::Backspace => {
                        if self.theme_edit.cursor > 0 {
                            self.theme_edit.cursor -= 1;
                            self.theme_edit.name.remove(self.theme_edit.cursor);
                        }
                    }
                    KeyCode::Left => {
                        self.theme_edit.cursor = self.theme_edit.cursor.saturating_sub(1);
                    }
                    KeyCode::Right => {
                        self.theme_edit.cursor = (self.theme_edit.cursor + 1).min(self.theme_edit.name.len());
                    }
                    KeyCode::Char(c) if !ctrl => {
                        if self.theme_edit.name.len() < 20 {
                            self.theme_edit.name.insert(self.theme_edit.cursor, c);
                            self.theme_edit.cursor += 1;
                        }
                    }
                    _ => {}
                }
                return;
            }

            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.theme_edit.active = false;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    self.theme_edit.slot = (self.theme_edit.slot + 1).min(5);
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.theme_edit.slot = self.theme_edit.slot.saturating_sub(1);
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    self.theme_edit.colors[self.theme_edit.slot] =
                        self.theme_edit.colors[self.theme_edit.slot].wrapping_add(1);
                }
                KeyCode::Char('h') | KeyCode::Left => {
                    self.theme_edit.colors[self.theme_edit.slot] =
                        self.theme_edit.colors[self.theme_edit.slot].wrapping_sub(1);
                }
                KeyCode::Char('L') => {
                    self.theme_edit.colors[self.theme_edit.slot] =
                        self.theme_edit.colors[self.theme_edit.slot].wrapping_add(10);
                }
                KeyCode::Char('H') => {
                    self.theme_edit.colors[self.theme_edit.slot] =
                        self.theme_edit.colors[self.theme_edit.slot].wrapping_sub(10);
                }
                KeyCode::Enter | KeyCode::Char('s') | KeyCode::Char('S') => {
                    self.theme_edit.naming = true;
                    self.theme_edit.name.clear();
                    self.theme_edit.cursor = 0;
                }
                _ => {}
            }
            return;
        }

        if self.drill.mode == ViewMode::DrillDown {
            match key.code {
                KeyCode::Esc | KeyCode::Backspace => {
                    if self.drill.path.len() > 1 {
                        self.drill.path.pop();
                        let parent = self.drill_current_path();
                        self.start_drill_scan(&parent);
                    } else {
                        self.drill.mode = ViewMode::Disks;
                        self.drill.path.clear();
                        self.drill.entries.clear();
                    }
                }
                KeyCode::Enter => {
                    if !self.drill.scanning {
                        if let Some(entry) = self.drill.entries.get(self.drill.selected) {
                            if entry.is_dir {
                                let path = entry.path.clone();
                                self.drill.path.push(path.clone());
                                self.start_drill_scan(&path);
                            }
                        }
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if !self.drill.entries.is_empty() {
                        self.drill.selected = (self.drill.selected + 1).min(self.drill.entries.len() - 1);
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.drill.selected = self.drill.selected.saturating_sub(1);
                }
                KeyCode::Home | KeyCode::Char('g') => {
                    self.drill.selected = 0;
                }
                KeyCode::End | KeyCode::Char('G') => {
                    if !self.drill.entries.is_empty() {
                        self.drill.selected = self.drill.entries.len() - 1;
                    }
                }
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    if self.drill.sort == DrillSortMode::Size {
                        self.drill.sort_rev = !self.drill.sort_rev;
                    } else {
                        self.drill.sort = DrillSortMode::Size;
                        self.drill.sort_rev = false;
                    }
                    self.sort_drill_entries();
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    if self.drill.sort == DrillSortMode::Name {
                        self.drill.sort_rev = !self.drill.sort_rev;
                    } else {
                        self.drill.sort = DrillSortMode::Name;
                        self.drill.sort_rev = false;
                    }
                    self.sort_drill_entries();
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    self.drill.sort_rev = !self.drill.sort_rev;
                    self.sort_drill_entries();
                }
                KeyCode::Char('o') | KeyCode::Char('O') => {
                    let path = self.drill_current_path();
                    if !self.test_mode {
                        #[cfg(target_os = "macos")]
                        { let _ = std::process::Command::new("open").arg(&path).spawn(); }
                        #[cfg(target_os = "linux")]
                        { let _ = std::process::Command::new("xdg-open").arg(&path).spawn(); }
                    }
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
                // Open theme chooser popup
                let themes = self.all_themes();
                // Pre-select current theme
                let current_key = if let Some(ref name) = self.prefs.active_theme {
                    name.clone()
                } else {
                    format!("{:?}", self.prefs.color_mode).to_lowercase()
                };
                self.theme_chooser.selected = themes.iter()
                    .position(|(k, _)| *k == current_key)
                    .unwrap_or(0);
                self.theme_chooser.orig_color_mode = self.prefs.color_mode;
                self.theme_chooser.orig_active_theme = self.prefs.active_theme.clone();
                self.theme_chooser.active = true;
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
                self.theme_edit.colors = [
                    idx(current.0), idx(current.1), idx(current.2),
                    idx(current.3), idx(current.4), idx(current.5),
                ];
                self.theme_edit.slot = 0;
                self.theme_edit.active = true;
                self.theme_edit.naming = false;
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
                self.filter.active = true;
                self.filter.prev = self.filter.text.clone();
                self.filter.buf = self.filter.text.clone();
                self.filter.cursor = self.filter.buf.len();
            }
            KeyCode::Char('0') => {
                self.filter.text.clear();
                self.filter.buf.clear();
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
                        self.drill.mode = ViewMode::DrillDown;
                        self.drill.path = vec![mount.clone()];
                        self.drill.selected = 0;
                        self.start_drill_scan(&mount);
                    }
                }
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                if let Some(idx) = self.selected {
                    let disks = self.sorted_disks();
                    if let Some(disk) = disks.get(idx) {
                        let mount = disk.mount.clone();
                        if !self.test_mode {
                            #[cfg(target_os = "macos")]
                            { let _ = std::process::Command::new("open").arg(&mount).spawn(); }
                            #[cfg(target_os = "linux")]
                            { let _ = std::process::Command::new("xdg-open").arg(&mount).spawn(); }
                        }
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
                for d in disks {
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
                        match copy_to_clipboard(&mount) {
                            Ok(_) => self.status_msg = Some((format!("Copied: {}", mount), Instant::now())),
                            Err(e) => self.status_msg = Some((format!("Copy failed: {}", e), Instant::now())),
                        }
                    }
                } else {
                    self.status_msg = Some(("Select a disk first (j/k)".into(), Instant::now()));
                }
            }
            KeyCode::Char('B') => {
                if let Some(idx) = self.selected {
                    let disks = self.sorted_disks();
                    if let Some(disk) = disks.get(idx) {
                        let mount = disk.mount.clone();
                        if let Some(pos) = self.prefs.bookmarks.iter().position(|b| *b == mount) {
                            self.prefs.bookmarks.remove(pos);
                            self.status_msg = Some((format!("Unpinned {}", mount), Instant::now()));
                        } else {
                            self.prefs.bookmarks.push(mount.clone());
                            self.status_msg = Some((format!("Pinned \u{2605} {}", mount), Instant::now()));
                        }
                        self.save();
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

    pub fn handle_mouse(&mut self, event: MouseEvent, term_w: u16, term_h: u16) {
        // Theme chooser mouse handling
        if self.theme_chooser.active {
            let themes = self.all_themes();
            let box_w: u16 = 50u16.min(term_w.saturating_sub(4));
            let box_h: u16 = (themes.len() as u16 + 4).min(term_h.saturating_sub(4));
            let x0 = (term_w.saturating_sub(box_w)) / 2;
            let y0 = (term_h.saturating_sub(box_h)) / 2;
            let content_start = y0 + 2;
            let content_end = y0 + box_h - 2;
            let visible = (content_end - content_start) as usize;
            let scroll = if self.theme_chooser.selected >= visible {
                self.theme_chooser.selected - visible + 1
            } else {
                0
            };

            match event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    let x = event.column;
                    let y = event.row;
                    // Click inside content area
                    if x > x0 && x < x0 + box_w - 1 && y >= content_start && y < content_end {
                        let clicked_idx = scroll + (y - content_start) as usize;
                        if clicked_idx < themes.len() {
                            self.theme_chooser.selected = clicked_idx;
                            self.apply_selected_theme();
                        }
                    } else if x < x0 || x >= x0 + box_w || y < y0 || y >= y0 + box_h {
                        // Click outside popup: cancel
                        self.prefs.color_mode = self.theme_chooser.orig_color_mode;
                        self.prefs.active_theme = self.theme_chooser.orig_active_theme.clone();
                        self.theme_chooser.active = false;
                    }
                }
                MouseEventKind::ScrollDown => {
                    let count = themes.len();
                    if count > 0 {
                        self.theme_chooser.selected = (self.theme_chooser.selected + 1).min(count - 1);
                        self.apply_selected_theme();
                    }
                }
                MouseEventKind::ScrollUp => {
                    self.theme_chooser.selected = self.theme_chooser.selected.saturating_sub(1);
                    self.apply_selected_theme();
                }
                _ => {}
            }
            return;
        }

        let show_border = self.prefs.show_border;
        let lm: u16 = if show_border { 1 } else { 0 };
        let rm: u16 = if show_border { 1 } else { 0 };
        let inner_w = term_w.saturating_sub(lm + rm);

        let mount_w = mount_col_width(inner_w, &self.prefs);
        let mount_sep_x = lm + 3 + mount_w as u16;

        let right_w = right_col_width(self);
        let bar_end_x = term_w.saturating_sub(rm + right_w + 1);
        let pct_w: u16 = if self.prefs.col_pct_w > 0 { self.prefs.col_pct_w } else { 5 };
        let right_start = term_w.saturating_sub(rm + right_w);
        let pct_sep_x = right_start + pct_w;

        let header_row: u16 = if show_border { 3 } else { 2 };

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let x = event.column;
                let y = event.row;

                // Column separator drag detection (checked first, before header sort)
                if x.abs_diff(mount_sep_x) <= 1 {
                    self.drag = Some(DragTarget::MountSep);
                } else if self.prefs.show_used && x.abs_diff(pct_sep_x) <= 1 {
                    self.drag = Some(DragTarget::PctSep);
                } else if x.abs_diff(bar_end_x) <= 1 {
                    self.drag = Some(DragTarget::BarEndSep);
                } else if self.prefs.show_header && y == header_row {
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
                    }
                } else {
                    // Click on disk row to select
                    let first_disk_row: u16 = if show_border { 1 } else { 0 }
                        + 2 // title + separator
                        + if self.prefs.show_header { 2 } else { 0 };
                    if y >= first_disk_row {
                        let disk_idx = (y - first_disk_row) as usize;
                        let count = self.sorted_disks().len();
                        if disk_idx < count {
                            if self.selected == Some(disk_idx) {
                                // Click again on selected: drill down
                                let disks = self.sorted_disks();
                                if let Some(disk) = disks.get(disk_idx) {
                                    let mount = disk.mount.clone();
                                    self.drill.mode = ViewMode::DrillDown;
                                    self.drill.path = vec![mount.clone()];
                                    self.drill.selected = 0;
                                    self.start_drill_scan(&mount);
                                }
                            } else {
                                self.selected = Some(disk_idx);
                            }
                        }
                    }
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
                // Right-click triggers instant hover tooltip at click position
                self.hover.pos = Some((event.column, event.row));
                self.hover.since = Some(Instant::now() - std::time::Duration::from_secs(2));
                self.hover.right_click = true;
            }
            MouseEventKind::Moved => {
                let new_pos = (event.column, event.row);
                if self.hover.pos != Some(new_pos) {
                    self.hover.pos = Some(new_pos);
                    self.hover.since = Some(Instant::now());
                    self.hover.right_click = false;
                }
            }
            MouseEventKind::ScrollDown => {
                if self.drill.mode == ViewMode::DrillDown {
                    if !self.drill.entries.is_empty() {
                        self.drill.selected = (self.drill.selected + 1).min(self.drill.entries.len() - 1);
                    }
                } else {
                    let count = self.sorted_disks().len();
                    if count > 0 {
                        self.selected = Some(match self.selected {
                            Some(i) => (i + 1).min(count - 1),
                            None => 0,
                        });
                    }
                }
            }
            MouseEventKind::ScrollUp => {
                if self.drill.mode == ViewMode::DrillDown {
                    self.drill.selected = self.drill.selected.saturating_sub(1);
                } else {
                    let count = self.sorted_disks().len();
                    if count > 0 {
                        self.selected = Some(match self.selected {
                            Some(i) => i.saturating_sub(1),
                            None => 0,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}
