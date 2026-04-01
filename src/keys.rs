use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::path::PathBuf;
use std::time::Instant;

use crate::app::{App, copy_to_clipboard};
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
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.quit = true;
                }
                KeyCode::Char('h')
                | KeyCode::Char('H')
                | KeyCode::Esc
                | KeyCode::Char('j')
                | KeyCode::Char('k') => {
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
                        self.theme_chooser.selected =
                            (self.theme_chooser.selected + 1).min(count - 1);
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
                            self.prefs.custom_themes.insert(
                                name.clone(),
                                ThemeColors {
                                    blue: colors[0],
                                    green: colors[1],
                                    purple: colors[2],
                                    light_purple: colors[3],
                                    royal: colors[4],
                                    dark_purple: colors[5],
                                },
                            );
                            self.prefs.active_theme = Some(name.clone());
                            self.save();
                            self.status_msg =
                                Some((format!("Saved theme: {}", name), Instant::now()));
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
                        self.theme_edit.cursor =
                            (self.theme_edit.cursor + 1).min(self.theme_edit.name.len());
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
                    if !self.drill.scanning
                        && let Some(entry) = self.drill.entries.get(self.drill.selected)
                        && entry.is_dir
                    {
                        let path = entry.path.clone();
                        self.drill.path.push(path.clone());
                        self.start_drill_scan(&path);
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if !self.drill.entries.is_empty() {
                        self.drill.selected =
                            (self.drill.selected + 1).min(self.drill.entries.len() - 1);
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
                        {
                            let _ = std::process::Command::new("open").arg(&path).spawn();
                        }
                        #[cfg(target_os = "linux")]
                        {
                            let _ = std::process::Command::new("xdg-open").arg(&path).spawn();
                        }
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
                self.theme_chooser.selected = themes
                    .iter()
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
                    idx(current.0),
                    idx(current.1),
                    idx(current.2),
                    idx(current.3),
                    idx(current.4),
                    idx(current.5),
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
                self.prefs.show_tooltips = !self.prefs.show_tooltips;
                self.save();
            }
            KeyCode::Char('z') | KeyCode::Char('Z') => {
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
                            {
                                let _ = std::process::Command::new("open").arg(&mount).spawn();
                            }
                            #[cfg(target_os = "linux")]
                            {
                                let _ = std::process::Command::new("xdg-open").arg(&mount).spawn();
                            }
                        }
                        self.status_msg = Some((format!("Opened {}", mount), Instant::now()));
                    }
                }
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                let disks = self.sorted_disks();
                let mut out = String::from("DISK MATRIX EXPORT\n");
                out.push_str(&format!(
                    "Host: {}  Date: {} {}\n\n",
                    self.stats.hostname,
                    chrono_now().0,
                    chrono_now().1
                ));
                out.push_str(&format!(
                    "{:<30} {:>5} {:>10} {:>10}\n",
                    "MOUNT", "PCT", "USED", "TOTAL"
                ));
                out.push_str(&format!("{}\n", "-".repeat(60)));
                for d in disks {
                    out.push_str(&format!(
                        "{:<30} {:>4.0}% {:>10} {:>10}\n",
                        d.mount,
                        d.pct,
                        format_bytes(d.used, self.prefs.unit_mode),
                        format_bytes(d.total, self.prefs.unit_mode),
                    ));
                }
                let path = dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".storageshower.export.txt");
                match std::fs::write(&path, &out) {
                    Ok(_) => {
                        self.status_msg =
                            Some((format!("Exported to {}", path.display()), Instant::now()))
                    }
                    Err(e) => {
                        self.status_msg = Some((format!("Export failed: {}", e), Instant::now()))
                    }
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Some(idx) = self.selected {
                    let disks = self.sorted_disks();
                    if let Some(disk) = disks.get(idx) {
                        let mount = disk.mount.clone();
                        match copy_to_clipboard(&mount) {
                            Ok(_) => {
                                self.status_msg =
                                    Some((format!("Copied: {}", mount), Instant::now()))
                            }
                            Err(e) => {
                                self.status_msg =
                                    Some((format!("Copy failed: {}", e), Instant::now()))
                            }
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
                            self.status_msg =
                                Some((format!("Pinned \u{2605} {}", mount), Instant::now()));
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
}

#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::prefs::Prefs;
    use crate::testutil::*;
    use crate::types::*;
    use crossterm::event::KeyCode;
    use std::sync::{Arc, Mutex};

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
    fn key_upper_t_toggles_tooltips() {
        let mut app = test_app();
        assert!(app.prefs.show_tooltips);
        app.handle_key(make_key(KeyCode::Char('T')));
        assert!(!app.prefs.show_tooltips);
        app.handle_key(make_key(KeyCode::Char('T')));
        assert!(app.prefs.show_tooltips);
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
    fn help_q_quits_app() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Char('q')));
        assert!(app.quit);
    }

    #[test]
    fn help_dismisses_with_k() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Char('k')));
        assert!(!app.show_help);
    }

    #[test]
    fn help_upper_q_quits_app() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key(make_key(KeyCode::Char('Q')));
        assert!(app.quit);
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
        for c in [
            's', 'n', 'u', 'r', 'c', 'i', 'v', 'd', 'g', 'x', 'w', 'm', 'f', 't', 'T', 'a', 'l',
            'p', '/', '0', '?',
        ] {
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
        app.handle_key(make_key(KeyCode::Char('t')));
        assert_eq!(app.prefs.thresh_warn, 60);
        app.handle_key(make_key(KeyCode::Char('t')));
        assert_eq!(app.prefs.thresh_warn, 70);
        app.handle_key(make_key(KeyCode::Char('t')));
        assert_eq!(app.prefs.thresh_warn, 80);
        app.handle_key(make_key(KeyCode::Char('t')));
        assert_eq!(app.prefs.thresh_warn, 50);
    }

    #[test]
    fn crit_threshold_full_cycle() {
        let mut app = test_app();
        app.prefs.thresh_crit = 80;
        app.handle_key(make_key(KeyCode::Char('z')));
        assert_eq!(app.prefs.thresh_crit, 85);
        app.handle_key(make_key(KeyCode::Char('z')));
        assert_eq!(app.prefs.thresh_crit, 90);
        app.handle_key(make_key(KeyCode::Char('z')));
        assert_eq!(app.prefs.thresh_crit, 95);
        app.handle_key(make_key(KeyCode::Char('z')));
        assert_eq!(app.prefs.thresh_crit, 80);
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
        app.handle_key(make_key(KeyCode::Char('z')));
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
}
