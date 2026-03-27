// storageshower — Cyberpunk disk usage TUI
// Single-file implementation with all features.

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    style::{Color, Modifier, Style},
    DefaultTerminal, Frame,
};
use serde::{Deserialize, Serialize};
use std::{
    io,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use sysinfo::{DiskKind, Disks, System};

// ─── Color Constants ───────────────────────────────────────────────────────

const DARK_BG: Color = Color::Indexed(234);
const HELP_BG: Color = Color::Indexed(236);
const DIM_BORDER: Color = Color::Indexed(240);

// ─── Types ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SortMode {
    Name,
    Pct,
    Size,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum BarStyle {
    Gradient,
    Solid,
    Thin,
    Ascii,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ColorMode {
    Default,
    Green,
    Blue,
    Purple,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum UnitMode {
    Human,
    GiB,
    MiB,
    Bytes,
}

#[derive(Clone)]
struct DiskEntry {
    mount: String,
    used: u64,
    total: u64,
    pct: f64,
    kind: DiskKind,
    fs: String,
}

#[derive(Clone)]
struct SysStats {
    hostname: String,
    load_avg: (f64, f64, f64),
    mem_used: u64,
    mem_total: u64,
    cpu_count: usize,
    process_count: usize,
    swap_used: u64,
    swap_total: u64,
    kernel: String,
    arch: String,
    uptime: u64,
    os_name: String,
    os_version: String,
}

impl Default for SysStats {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            load_avg: (0.0, 0.0, 0.0),
            mem_used: 0,
            mem_total: 1,
            cpu_count: 0,
            process_count: 0,
            swap_used: 0,
            swap_total: 0,
            kernel: String::new(),
            arch: String::new(),
            uptime: 0,
            os_name: String::new(),
            os_version: String::new(),
        }
    }
}

// ─── Preferences ───────────────────────────────────────────────────────────

#[derive(Clone, Serialize, Deserialize)]
struct Prefs {
    sort_mode: SortMode,
    sort_rev: bool,
    show_local: bool,
    refresh_rate: u64,
    bar_style: BarStyle,
    color_mode: ColorMode,
    thresh_warn: u8,
    thresh_crit: u8,
    show_bars: bool,
    show_border: bool,
    show_header: bool,
    compact: bool,
    show_used: bool,
    full_mount: bool,
}

impl Default for Prefs {
    fn default() -> Self {
        Self {
            sort_mode: SortMode::Name,
            sort_rev: false,
            show_local: false,
            refresh_rate: 1,
            bar_style: BarStyle::Gradient,
            color_mode: ColorMode::Default,
            thresh_warn: 70,
            thresh_crit: 90,
            show_bars: true,
            show_border: true,
            show_header: true,
            compact: false,
            show_used: true,
            full_mount: false,
        }
    }
}

fn prefs_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".storageshower.conf")
}

fn load_prefs() -> Prefs {
    std::fs::read_to_string(prefs_path())
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_prefs(p: &Prefs) {
    if let Ok(s) = toml::to_string_pretty(p) {
        let _ = std::fs::write(prefs_path(), s);
    }
}

// ─── App State ─────────────────────────────────────────────────────────────

struct App {
    prefs: Prefs,
    disks: Vec<DiskEntry>,
    stats: SysStats,
    shared_stats: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>,
    paused: bool,
    show_help: bool,
    filter: String,
    filter_mode: bool,
    filter_buf: String,
    unit_mode: UnitMode,
    quit: bool,
}

impl App {
    fn new(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>) -> Self {
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
            unit_mode: UnitMode::Human,
            quit: false,
        }
    }

    fn refresh_data(&mut self) {
        if self.paused {
            return;
        }
        let (stats, disks) = self.shared_stats.lock().unwrap().clone();
        self.stats = stats;
        self.disks = disks;
    }

    fn sorted_disks(&self) -> Vec<DiskEntry> {
        let mut ds: Vec<DiskEntry> = self.disks.clone();
        if self.prefs.show_local {
            ds.retain(|d| {
                matches!(d.kind, DiskKind::HDD | DiskKind::SSD)
                    || (!d.mount.starts_with("/sys")
                        && !d.mount.starts_with("/proc")
                        && !d.mount.starts_with("/dev/shm")
                        && !d.mount.starts_with("/run")
                        && !d.mount.starts_with("/snap")
                        && d.fs != "tmpfs"
                        && d.fs != "devtmpfs"
                        && d.fs != "squashfs"
                        && d.fs != "overlay"
                        && d.total > 0)
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

    fn save(&self) {
        save_prefs(&self.prefs);
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if self.filter_mode {
            match key.code {
                KeyCode::Enter => {
                    self.filter = self.filter_buf.clone();
                    self.filter_mode = false;
                }
                KeyCode::Esc => {
                    self.filter_mode = false;
                    self.filter_buf.clear();
                }
                KeyCode::Backspace => {
                    self.filter_buf.pop();
                }
                KeyCode::Char(c) => {
                    self.filter_buf.push(c);
                }
                _ => {}
            }
            return;
        }

        if self.show_help {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Esc => {
                    self.show_help = false;
                }
                _ => {}
            }
            return;
        }

        match key.code {
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
                self.save();
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.prefs.show_header = !self.prefs.show_header;
                self.save();
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                self.prefs.show_border = !self.prefs.show_border;
                self.save();
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                self.prefs.compact = !self.prefs.compact;
                self.save();
            }
            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.prefs.full_mount = !self.prefs.full_mount;
                self.save();
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.unit_mode = match self.unit_mode {
                    UnitMode::Human => UnitMode::GiB,
                    UnitMode::GiB => UnitMode::MiB,
                    UnitMode::MiB => UnitMode::Bytes,
                    UnitMode::Bytes => UnitMode::Human,
                };
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
                self.filter_buf = self.filter.clone();
            }
            KeyCode::Char('0') => {
                self.filter.clear();
                self.filter_buf.clear();
            }
            _ => {}
        }
    }
}

// ─── Helpers ───────────────────────────────────────────────────────────────

fn format_bytes(b: u64, mode: UnitMode) -> String {
    match mode {
        UnitMode::Bytes => format!("{b}B"),
        UnitMode::MiB => format!("{:.1}M", b as f64 / 1_048_576.0),
        UnitMode::GiB => format!("{:.1}G", b as f64 / 1_073_741_824.0),
        UnitMode::Human => {
            if b >= 1_099_511_627_776 {
                format!("{:.1}T", b as f64 / 1_099_511_627_776.0)
            } else if b >= 1_073_741_824 {
                format!("{:.1}G", b as f64 / 1_073_741_824.0)
            } else if b >= 1_048_576 {
                format!("{:.1}M", b as f64 / 1_048_576.0)
            } else if b >= 1024 {
                format!("{:.1}K", b as f64 / 1024.0)
            } else {
                format!("{b}B")
            }
        }
    }
}

fn format_uptime(secs: u64) -> String {
    let d = secs / 86400;
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    if d > 0 {
        format!("{d}d{h}h{m}m")
    } else if h > 0 {
        format!("{h}h{m}m")
    } else {
        format!("{m}m")
    }
}

fn palette(mode: ColorMode) -> (Color, Color, Color, Color, Color, Color) {
    // Returns (blue, green, purple, lpurple, royal, dpurple)
    match mode {
        ColorMode::Default => (
            Color::Indexed(27),
            Color::Indexed(48),
            Color::Indexed(135),
            Color::Indexed(141),
            Color::Indexed(63),
            Color::Indexed(99),
        ),
        ColorMode::Green => (
            Color::Indexed(28),
            Color::Indexed(46),
            Color::Indexed(34),
            Color::Indexed(40),
            Color::Indexed(22),
            Color::Indexed(35),
        ),
        ColorMode::Blue => (
            Color::Indexed(19),
            Color::Indexed(39),
            Color::Indexed(25),
            Color::Indexed(33),
            Color::Indexed(21),
            Color::Indexed(32),
        ),
        ColorMode::Purple => (
            Color::Indexed(91),
            Color::Indexed(177),
            Color::Indexed(128),
            Color::Indexed(134),
            Color::Indexed(93),
            Color::Indexed(97),
        ),
    }
}

fn border_color(app: &App) -> Color {
    let (blue, ..) = palette(app.prefs.color_mode);
    if app.paused {
        DIM_BORDER
    } else {
        blue
    }
}

fn thresh_color(pct: f64, app: &App) -> (Color, Option<Color>, &'static str) {
    let (_, green, _, lpurple, royal, _) = palette(app.prefs.color_mode);
    if pct >= app.prefs.thresh_crit as f64 {
        (royal, Some(royal), "\u{2716}") // ✖
    } else if pct >= app.prefs.thresh_warn as f64 {
        (lpurple, Some(lpurple), "\u{26A0}") // ⚠
    } else {
        (green, None, "\u{25C8}") // ◈
    }
}

fn gradient_color_at(frac: f64, mode: ColorMode) -> Color {
    let (blue, green, purple, _, _, dpurple) = palette(mode);
    if frac < 0.33 {
        green
    } else if frac < 0.55 {
        blue
    } else if frac < 0.80 {
        purple
    } else {
        dpurple
    }
}

fn truncate_mount(mount: &str, width: usize) -> String {
    if mount.chars().count() <= width {
        format!("{:<width$}", mount, width = width)
    } else {
        let s: String = mount.chars().take(width.saturating_sub(1)).collect();
        format!("{}\u{2026}", s) // …
    }
}

fn mount_col_width(inner_w: u16, prefs: &Prefs) -> usize {
    if prefs.compact {
        16
    } else {
        (inner_w as usize / 3).max(12)
    }
}

// ─── Low-level buffer writing ──────────────────────────────────────────────

fn set_cell(buf: &mut Buffer, x: u16, y: u16, ch: &str, style: Style) {
    let area = buf.area();
    if x < area.x + area.width && y < area.y + area.height && x >= area.x && y >= area.y {
        let cell = &mut buf[(x, y)];
        cell.set_symbol(ch);
        cell.set_style(style);
    }
}

fn set_str(buf: &mut Buffer, x: u16, y: u16, s: &str, style: Style, max_w: u16) {
    let mut cx = x;
    for ch in s.chars() {
        if cx >= x.saturating_add(max_w) {
            break;
        }
        set_cell(buf, cx, y, &ch.to_string(), style);
        cx += 1;
    }
}

// ─── Rendering ─────────────────────────────────────────────────────────────

fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let w = area.width;
    let h = area.height;
    if w < 40 || h < 10 {
        let buf = frame.buffer_mut();
        set_str(buf, 0, 0, "Terminal too small (need 40x10)", Style::default().fg(Color::Red), w);
        return;
    }

    let buf = frame.buffer_mut();
    let bc = border_color(app);
    let border_s = Style::default().fg(bc);
    let (pal_blue, pal_green, _pal_purple, pal_lpurple, _pal_royal, _pal_dpurple) =
        palette(app.prefs.color_mode);

    let show_border = app.prefs.show_border;
    let lm: u16 = if show_border { 1 } else { 0 };
    let rm: u16 = if show_border { 1 } else { 0 };
    let inner_w = w.saturating_sub(lm + rm);

    // Clear background
    for y in 0..h {
        for x in 0..w {
            let cell = &mut buf[(x, y)];
            cell.set_symbol(" ");
            cell.set_style(Style::default());
        }
    }

    // Top border
    if show_border {
        set_cell(buf, 0, 0, "\u{2554}", border_s); // ╔
        for x in 1..w - 1 {
            set_cell(buf, x, 0, "\u{2550}", border_s); // ═
        }
        set_cell(buf, w - 1, 0, "\u{2557}", border_s); // ╗
    }

    // Bottom border
    if show_border {
        set_cell(buf, 0, h - 1, "\u{255A}", border_s); // ╚
        for x in 1..w - 1 {
            set_cell(buf, x, h - 1, "\u{2550}", border_s); // ═
        }
        set_cell(buf, w - 1, h - 1, "\u{255D}", border_s); // ╝
    }

    // Side borders
    if show_border {
        for y in 1..h - 1 {
            set_cell(buf, 0, y, "\u{2551}", border_s); // ║
            set_cell(buf, w - 1, y, "\u{2551}", border_s); // ║
        }
    }

    let mut row: u16 = if show_border { 1 } else { 0 };

    // ─── Title banner ───
    {
        let banner_s = Style::default().fg(pal_green).bg(DARK_BG);
        let accent_s = Style::default().fg(pal_blue).bg(DARK_BG).add_modifier(Modifier::BOLD);

        // Fill banner bg
        for x in lm..w.saturating_sub(rm) {
            set_cell(buf, x, row, " ", Style::default().bg(DARK_BG));
        }

        let now = chrono_now();
        let hostname = &app.stats.hostname;
        let mut title = format!(
            " \u{25B6}\u{25B6}\u{25B6} DISK MATRIX \u{25C0}\u{25C0}\u{25C0} node:{} date:{} clock:{}",
            hostname, now.0, now.1
        );
        if app.paused {
            title.push_str(" \u{23F8} PAUSED");
        }
        if app.prefs.show_local {
            title.push_str(" [local]");
        }
        if !app.filter.is_empty() {
            title.push_str(&format!(" [filter:{}]", app.filter));
        }

        // Adaptive header stats
        let load = app.stats.load_avg;
        let mem_pct = if app.stats.mem_total > 0 {
            (app.stats.mem_used as f64 / app.stats.mem_total as f64) * 100.0
        } else {
            0.0
        };
        let load_str = format!(
            " load:{:.2}/{:.2}/{:.2} mem:{}/{}({:.0}%) cpu:{}",
            load.0, load.1, load.2,
            format_bytes(app.stats.mem_used, UnitMode::Human),
            format_bytes(app.stats.mem_total, UnitMode::Human),
            mem_pct,
            app.stats.cpu_count
        );
        title.push_str(&load_str);

        if inner_w > 100 {
            title.push_str(&format!(" procs:{}", app.stats.process_count));
        }
        if inner_w > 120 {
            title.push_str(&format!(
                " swap:{}/{}",
                format_bytes(app.stats.swap_used, UnitMode::Human),
                format_bytes(app.stats.swap_total, UnitMode::Human)
            ));
        }
        if inner_w > 140 && !app.stats.kernel.is_empty() {
            title.push_str(&format!(" kern:{}", app.stats.kernel));
        }
        if inner_w > 160 && !app.stats.arch.is_empty() {
            title.push_str(&format!(" arch:{}", app.stats.arch));
        }

        let help_hint = " \u{2502} h=help ";
        let avail = inner_w as usize;
        if title.chars().count() + help_hint.len() < avail {
            let pad = avail - title.chars().count() - help_hint.len();
            title.push_str(&" ".repeat(pad));
            title.push_str(help_hint);
        }

        let title_display: String = title.chars().take(inner_w as usize).collect();
        set_str(buf, lm, row, &title_display, banner_s, inner_w);

        // Colorize DISK MATRIX
        if let Some(idx) = title_display.find("DISK MATRIX") {
            // Count chars before the match to get proper x position
            let char_offset = title_display[..idx].chars().count() as u16;
            set_str(buf, lm + char_offset, row, "DISK MATRIX", accent_s, 11);
        }

        row += 1;
    }

    // ─── Header separator ───
    draw_separator(buf, row, w, show_border, border_s);
    row += 1;

    // ─── Column headers ───
    if app.prefs.show_header {
        let hdr_s = Style::default().fg(pal_lpurple).add_modifier(Modifier::BOLD);

        for x in lm..w.saturating_sub(rm) {
            set_cell(buf, x, row, " ", Style::default());
        }

        let mount_w = mount_col_width(inner_w, &app.prefs);
        let sort_arrow = if app.prefs.sort_rev { "\u{25BC}" } else { "\u{25B2}" }; // ▼ ▲

        let name_arrow = if app.prefs.sort_mode == SortMode::Name { sort_arrow } else { " " };
        let pct_arrow = if app.prefs.sort_mode == SortMode::Pct { sort_arrow } else { " " };
        let size_arrow = if app.prefs.sort_mode == SortMode::Size { sort_arrow } else { " " };

        let mount_hdr = format!(" MOUNT{}", name_arrow);
        set_str(buf, lm, row, &mount_hdr, hdr_s, (mount_w + 3) as u16);

        if app.prefs.show_bars {
            let bar_start = lm + 3 + mount_w as u16 + 1;
            set_str(buf, bar_start, row, "USAGE", hdr_s, 5);
        }

        if app.prefs.show_used {
            let right_start = w.saturating_sub(rm + 21);
            let pct_hdr = format!("PCT{} ", pct_arrow);
            set_str(buf, right_start, row, &pct_hdr, hdr_s, 5);
            let used_hdr = format!("USED/SIZE{}", size_arrow);
            set_str(buf, right_start + 5, row, &used_hdr, hdr_s, 16);
        } else {
            let right_start = w.saturating_sub(rm + 6);
            let pct_hdr = format!("PCT{}", pct_arrow);
            set_str(buf, right_start, row, &pct_hdr, hdr_s, 6);
        }

        row += 1;

        // Column separator
        draw_separator(buf, row, w, show_border, border_s);
        row += 1;
    }

    // ─── Footer area ───
    let footer_rows: u16 = 2 + (if show_border { 1 } else { 0 });
    let disk_area_end = h.saturating_sub(footer_rows);

    // ─── Disk rows ───
    let disks = app.sorted_disks();
    let disk_count = disks.len();
    let mount_w = mount_col_width(inner_w, &app.prefs);

    for disk in disks.iter() {
        if row >= disk_area_end {
            break;
        }

        let (fg_color, bg_pct, icon) = thresh_color(disk.pct, app);

        // Icon
        let icon_str = format!(" {} ", icon);
        set_str(buf, lm, row, &icon_str, Style::default().fg(fg_color), 3);

        // Mount name
        let mount_display = if app.prefs.full_mount {
            format!("{:<width$}", disk.mount, width = mount_w.saturating_sub(1))
        } else {
            truncate_mount(&disk.mount, mount_w.saturating_sub(1))
        };
        set_str(buf, lm + 3, row, &mount_display, Style::default().fg(pal_green), mount_w as u16);

        // Separator pipe
        let bar_col_start = lm + 3 + mount_w as u16;
        set_cell(buf, bar_col_start, row, "\u{2502}", border_s); // │

        // Bar
        if app.prefs.show_bars {
            let right_info_w: u16 = if app.prefs.show_used { 22 } else { 7 };
            let bar_end = w.saturating_sub(rm + right_info_w + 1);
            let bar_start = bar_col_start + 1;
            if bar_end > bar_start + 2 {
                let bar_w = (bar_end - bar_start) as usize;
                let filled = ((disk.pct / 100.0) * bar_w as f64).round() as usize;
                let filled = filled.min(bar_w);

                for j in 0..bar_w {
                    let x = bar_start + j as u16;
                    if x >= w.saturating_sub(rm) {
                        break;
                    }
                    if j < filled {
                        match app.prefs.bar_style {
                            BarStyle::Gradient => {
                                let frac = j as f64 / bar_w as f64;
                                let gc = gradient_color_at(frac, app.prefs.color_mode);
                                let ch = if j == filled - 1 {
                                    "\u{25B8}" // ▸
                                } else if frac < 0.33 {
                                    "\u{2588}" // █
                                } else if frac < 0.55 {
                                    "\u{2593}" // ▓
                                } else if frac < 0.80 {
                                    "\u{2592}" // ▒
                                } else {
                                    "\u{2591}" // ░
                                };
                                set_cell(buf, x, row, ch, Style::default().fg(gc));
                            }
                            BarStyle::Solid => {
                                set_cell(buf, x, row, "\u{2588}", Style::default().fg(fg_color));
                            }
                            BarStyle::Thin => {
                                if j == filled - 1 {
                                    set_cell(buf, x, row, "\u{25B8}", Style::default().fg(fg_color));
                                } else {
                                    set_cell(buf, x, row, "\u{25AC}", Style::default().fg(fg_color)); // ▬
                                }
                            }
                            BarStyle::Ascii => {
                                if j == filled - 1 {
                                    set_cell(buf, x, row, ">", Style::default().fg(fg_color));
                                } else {
                                    set_cell(buf, x, row, "#", Style::default().fg(fg_color));
                                }
                            }
                        }
                    } else {
                        let (ch, color) = match app.prefs.bar_style {
                            BarStyle::Gradient | BarStyle::Solid => (" ", Color::Indexed(240)),
                            BarStyle::Thin => ("\u{00B7}", Color::Indexed(240)), // ·
                            BarStyle::Ascii => ("-", Color::Indexed(240)),
                        };
                        set_cell(buf, x, row, ch, Style::default().fg(color));
                    }
                }

                // Bar end separator
                if bar_end < w.saturating_sub(rm) {
                    set_cell(buf, bar_end, row, "\u{2502}", border_s); // │
                }
            }
        }

        // Pct and size info
        let pct_str = format!("{:>3.0}%", disk.pct);
        let pct_style = if let Some(bg) = bg_pct {
            Style::default().fg(Color::White).bg(bg)
        } else {
            Style::default().fg(pal_green)
        };

        if app.prefs.show_used {
            let right_start = w.saturating_sub(rm + 21);
            set_str(buf, right_start, row, &pct_str, pct_style, 5);
            let size_str = format!(
                " {}/{}",
                format_bytes(disk.used, app.unit_mode),
                format_bytes(disk.total, app.unit_mode)
            );
            set_str(buf, right_start + 5, row, &size_str, Style::default().fg(pal_lpurple), 16);
        } else {
            let right_start = w.saturating_sub(rm + 6);
            set_str(buf, right_start, row, &pct_str, pct_style, 5);
        }

        row += 1;
    }

    // Fill empty bordered rows
    // (already blank from clear, side borders already drawn)

    // ─── Footer separator ───
    if disk_area_end < h {
        draw_separator(buf, disk_area_end, w, show_border, border_s);
    }

    // ─── Footer banner ───
    {
        let frow = disk_area_end + 1;
        if frow < h {
            let footer_s = Style::default().fg(pal_green).bg(DARK_BG);

            for x in lm..w.saturating_sub(rm) {
                set_cell(buf, x, frow, " ", Style::default().bg(DARK_BG));
            }

            let sort_name = match app.prefs.sort_mode {
                SortMode::Name => "name",
                SortMode::Pct => "pct",
                SortMode::Size => "size",
            };
            let sort_dir = if app.prefs.sort_rev { "\u{25BC}" } else { "\u{25B2}" };
            let bar_name = match app.prefs.bar_style {
                BarStyle::Gradient => "gradient",
                BarStyle::Solid => "solid",
                BarStyle::Thin => "thin",
                BarStyle::Ascii => "ascii",
            };
            let color_name = match app.prefs.color_mode {
                ColorMode::Default => "default",
                ColorMode::Green => "green",
                ColorMode::Blue => "blue",
                ColorMode::Purple => "purple",
            };
            let unit_name = match app.unit_mode {
                UnitMode::Human => "human",
                UnitMode::GiB => "GiB",
                UnitMode::MiB => "MiB",
                UnitMode::Bytes => "bytes",
            };

            let mut footer = format!(
                " \u{27E6}zpwr\u{22B7}cyberdeck\u{27E7} \u{25C0}\u{25C0}\u{25C0} vol:{} \u{2502} sort:{}{} \u{2502} {}s \u{2502} {} \u{2502} {} \u{2502} {}",
                disk_count, sort_name, sort_dir, app.prefs.refresh_rate, bar_name, color_name, unit_name
            );

            // Adaptive footer stats
            footer.push_str(&format!(" \u{2502} up:{}", format_uptime(app.stats.uptime)));

            if inner_w > 80 {
                if let Some(user) = get_username() {
                    footer.push_str(&format!(" \u{2502} user:{}", user));
                }
            }
            if inner_w > 95 {
                footer.push_str(&format!(" \u{2502} ip:{}", get_local_ip()));
            }
            if inner_w > 110 {
                footer.push_str(&format!(
                    " \u{2502} os:{}{}",
                    app.stats.os_name,
                    if app.stats.os_version.is_empty() {
                        String::new()
                    } else {
                        format!(" {}", app.stats.os_version)
                    }
                ));
            }
            if inner_w > 130 {
                if let Ok(shell) = std::env::var("SHELL") {
                    let shell_name = shell.rsplit('/').next().unwrap_or(&shell);
                    footer.push_str(&format!(" \u{2502} sh:{}", shell_name));
                }
            }
            if inner_w > 140 {
                if let Ok(tty) = get_tty() {
                    footer.push_str(&format!(" \u{2502} tty:{}", tty));
                }
            }
            if inner_w > 150 {
                footer.push_str(&format!(" \u{2502} disks:{}", disk_count));
            }
            if inner_w > 160 {
                if let Some(bat) = get_battery() {
                    footer.push_str(&format!(" \u{2502} bat:{}%", bat));
                }
            }
            if inner_w > 190 {
                footer.push_str(&format!(" \u{2502} {}x{}", w, h));
            }

            if app.filter_mode {
                footer.push_str(&format!(" \u{2502} FILTER> {}_", app.filter_buf));
            }

            let footer_display: String = footer.chars().take(inner_w as usize).collect();
            set_str(buf, lm, frow, &footer_display, footer_s, inner_w);
        }
    }

    // ─── Help overlay ───
    if app.show_help {
        draw_help(buf, w, h, app);
    }
}

fn draw_separator(buf: &mut Buffer, row: u16, w: u16, show_border: bool, border_s: Style) {
    if show_border {
        set_cell(buf, 0, row, "\u{2560}", border_s); // ╠
        for x in 1..w - 1 {
            set_cell(buf, x, row, "\u{2550}", border_s); // ═
        }
        set_cell(buf, w - 1, row, "\u{2563}", border_s); // ╣
    } else {
        for x in 0..w {
            set_cell(buf, x, row, "\u{2550}", border_s);
        }
    }
}

fn draw_help(buf: &mut Buffer, w: u16, h: u16, app: &App) {
    let box_w: u16 = 74u16.min(w.saturating_sub(4));
    let box_h: u16 = 34u16.min(h.saturating_sub(4));
    let x0 = (w.saturating_sub(box_w)) / 2;
    let y0 = (h.saturating_sub(box_h)) / 2;
    let bc = border_color(app);
    let border_s = Style::default().fg(bc);
    let bg_s = Style::default().fg(Color::White).bg(HELP_BG);
    let key_s = Style::default().fg(Color::Indexed(48)).bg(HELP_BG);
    let val_s = Style::default().fg(Color::Indexed(141)).bg(HELP_BG);
    let title_s = Style::default()
        .fg(Color::Indexed(27))
        .bg(HELP_BG)
        .add_modifier(Modifier::BOLD);
    let section_s = Style::default()
        .fg(Color::Indexed(99))
        .bg(HELP_BG)
        .add_modifier(Modifier::BOLD);

    // Fill background
    for y in y0..y0 + box_h {
        for x in x0..x0 + box_w {
            set_cell(buf, x, y, " ", Style::default().bg(HELP_BG));
        }
    }

    // Borders
    set_cell(buf, x0, y0, "\u{2554}", border_s);
    set_cell(buf, x0 + box_w - 1, y0, "\u{2557}", border_s);
    set_cell(buf, x0, y0 + box_h - 1, "\u{255A}", border_s);
    set_cell(buf, x0 + box_w - 1, y0 + box_h - 1, "\u{255D}", border_s);
    for x in x0 + 1..x0 + box_w - 1 {
        set_cell(buf, x, y0, "\u{2550}", border_s);
        set_cell(buf, x, y0 + box_h - 1, "\u{2550}", border_s);
    }
    for y in y0 + 1..y0 + box_h - 1 {
        set_cell(buf, x0, y, "\u{2551}", border_s);
        set_cell(buf, x0 + box_w - 1, y, "\u{2551}", border_s);
    }

    // Title
    let title = "\u{2328} DISK MATRIX \u{2014} KEYBOARD SHORTCUTS"; // ⌨ —
    let tlen = title.chars().count() as u16;
    let tx = x0 + (box_w.saturating_sub(tlen)) / 2;
    set_str(buf, tx, y0 + 1, title, title_s, box_w - 2);

    // Help entries in two columns
    struct HelpEntry {
        key: &'static str,
        desc: &'static str,
        val_fn: fn(&App) -> String,
        is_section: bool,
    }

    fn empty_val(_: &App) -> String { String::new() }

    let entries: Vec<HelpEntry> = vec![
        HelpEntry { key: "GENERAL", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "q/Q", desc: "Quit (close help first)", val_fn: empty_val, is_section: false },
        HelpEntry { key: "h/H", desc: "Toggle help overlay", val_fn: empty_val, is_section: false },
        HelpEntry { key: "p/P", desc: "Pause/resume", val_fn: |a| format!("[{}]", if a.paused {"paused"} else {"running"}), is_section: false },
        HelpEntry { key: "f/F", desc: "Cycle refresh rate", val_fn: |a| format!("[{}s]", a.prefs.refresh_rate), is_section: false },
        HelpEntry { key: "l/L", desc: "Local disks only", val_fn: |a| format!("[{}]", if a.prefs.show_local {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "SORT", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "n/N", desc: "Sort by name", val_fn: |a| if a.prefs.sort_mode == SortMode::Name {"[active]".into()} else {String::new()}, is_section: false },
        HelpEntry { key: "u/U", desc: "Sort by usage %", val_fn: |a| if a.prefs.sort_mode == SortMode::Pct {"[active]".into()} else {String::new()}, is_section: false },
        HelpEntry { key: "s/S", desc: "Sort by size", val_fn: |a| if a.prefs.sort_mode == SortMode::Size {"[active]".into()} else {String::new()}, is_section: false },
        HelpEntry { key: "r/R", desc: "Reverse sort", val_fn: |a| format!("[{}]", if a.prefs.sort_rev {"\u{25BC}"} else {"\u{25B2}"}), is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "DISPLAY", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "b", desc: "Cycle bar style", val_fn: |a| format!("[{}]", match a.prefs.bar_style { BarStyle::Gradient=>"gradient", BarStyle::Solid=>"solid", BarStyle::Thin=>"thin", BarStyle::Ascii=>"ascii" }), is_section: false },
        HelpEntry { key: "c", desc: "Cycle color mode", val_fn: |a| format!("[{}]", match a.prefs.color_mode { ColorMode::Default=>"default", ColorMode::Green=>"green", ColorMode::Blue=>"blue", ColorMode::Purple=>"purple" }), is_section: false },
        HelpEntry { key: "v/V", desc: "Toggle bars", val_fn: |a| format!("[{}]", if a.prefs.show_bars {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "d/D", desc: "Toggle used/size", val_fn: |a| format!("[{}]", if a.prefs.show_used {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "g/G", desc: "Toggle col headers", val_fn: |a| format!("[{}]", if a.prefs.show_header {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "x/X", desc: "Toggle border", val_fn: |a| format!("[{}]", if a.prefs.show_border {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "m/M", desc: "Compact mounts", val_fn: |a| format!("[{}]", if a.prefs.compact {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "w/W", desc: "Full mount paths", val_fn: |a| format!("[{}]", if a.prefs.full_mount {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "i/I", desc: "Cycle units", val_fn: |a| format!("[{}]", match a.unit_mode { UnitMode::Human=>"human", UnitMode::GiB=>"GiB", UnitMode::MiB=>"MiB", UnitMode::Bytes=>"bytes" }), is_section: false },
        HelpEntry { key: "t", desc: "Cycle warn threshold", val_fn: |a| format!("[{}%]", a.prefs.thresh_warn), is_section: false },
        HelpEntry { key: "T", desc: "Cycle crit threshold", val_fn: |a| format!("[{}%]", a.prefs.thresh_crit), is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "FILTER", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "/", desc: "Enter filter mode", val_fn: empty_val, is_section: false },
        HelpEntry { key: "0", desc: "Clear filter", val_fn: |a| if a.filter.is_empty() {String::new()} else {format!("[{}]", a.filter)}, is_section: false },
    ];

    let content_h = (box_h as usize).saturating_sub(4); // rows available for entries
    let half = (content_h + 1) / 2; // number of entries that fit in first column with some breathing room
    let col_w = ((box_w as usize).saturating_sub(4)) / 2;

    for (i, entry) in entries.iter().enumerate() {
        let (col, local_idx) = if i < half.min(entries.len()) {
            (0u16, i)
        } else {
            (1u16, i - half.min(entries.len()))
        };
        let ey = y0 + 3 + local_idx as u16;
        if ey >= y0 + box_h - 1 {
            continue;
        }
        let cx = x0 + 2 + col * col_w as u16;

        if entry.key.is_empty() && !entry.is_section {
            // Blank spacer
            continue;
        }

        if entry.is_section {
            set_str(buf, cx, ey, entry.key, section_s, col_w as u16);
        } else {
            set_str(buf, cx, ey, entry.key, key_s, 6);
            set_str(buf, cx + 7, ey, entry.desc, bg_s, 20);
            let val = (entry.val_fn)(app);
            if !val.is_empty() {
                let vw = col_w.saturating_sub(28);
                set_str(buf, cx + 28, ey, &val, val_s, vw as u16);
            }
        }
    }
}

// ─── Time helpers ──────────────────────────────────────────────────────────

fn chrono_now() -> (String, String) {
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (y, mo, d, h, mi, s) = epoch_to_local(epoch as i64);
    (
        format!("{:04}.{:02}.{:02}", y, mo, d),
        format!("{:02}:{:02}:{:02}", h, mi, s),
    )
}

#[cfg(unix)]
fn epoch_to_local(epoch: i64) -> (i32, u32, u32, u32, u32, u32) {
    unsafe {
        let mut tm: libc::tm = std::mem::zeroed();
        let t = epoch as libc::time_t;
        libc::localtime_r(&t, &mut tm);
        (
            tm.tm_year as i32 + 1900,
            tm.tm_mon as u32 + 1,
            tm.tm_mday as u32,
            tm.tm_hour as u32,
            tm.tm_min as u32,
            tm.tm_sec as u32,
        )
    }
}

#[cfg(not(unix))]
fn epoch_to_local(epoch: i64) -> (i32, u32, u32, u32, u32, u32) {
    // Fallback: UTC
    let secs_per_day = 86400i64;
    let mut days = epoch / secs_per_day;
    let day_secs = (epoch % secs_per_day) as u32;
    let hh = day_secs / 3600;
    let mm = (day_secs % 3600) / 60;
    let ss = day_secs % 60;
    let mut y = 1970i32;
    loop {
        let dy: i64 = if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 { 366 } else { 365 };
        if days < dy { break; }
        days -= dy;
        y += 1;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let mdays: [i64; 12] = [31, if leap {29} else {28}, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut mo = 1u32;
    for i in 0..12 {
        if days < mdays[i] { mo = i as u32 + 1; break; }
        days -= mdays[i];
    }
    (y, mo, days as u32 + 1, hh, mm, ss)
}

// ─── System info helpers ───────────────────────────────────────────────────

fn get_username() -> Option<String> {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .ok()
}

fn get_local_ip() -> String {
    std::net::UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| {
            s.connect("8.8.8.8:80")?;
            s.local_addr()
        })
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}

#[cfg(unix)]
fn get_tty() -> Result<String, ()> {
    unsafe {
        let name = libc::ttyname(0);
        if name.is_null() {
            Err(())
        } else {
            Ok(std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned())
        }
    }
}

#[cfg(not(unix))]
fn get_tty() -> Result<String, ()> {
    Err(())
}

fn get_battery() -> Option<u8> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("pmset")
            .args(["-g", "batt"])
            .output()
            .ok()
            .and_then(|o| {
                let s = String::from_utf8_lossy(&o.stdout);
                for word in s.split_whitespace() {
                    if word.ends_with("%;") || word.ends_with('%') {
                        let num: String = word.chars().take_while(|c| c.is_ascii_digit()).collect();
                        if let Ok(v) = num.parse::<u8>() {
                            return Some(v);
                        }
                    }
                }
                None
            })
    }
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .ok()
            .and_then(|s| s.trim().parse().ok())
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

// ─── Background stats collection ───────────────────────────────────────────

fn collect_disk_entries(disks: &Disks) -> Vec<DiskEntry> {
    disks
        .list()
        .iter()
        .map(|d| {
            let total = d.total_space();
            let avail = d.available_space();
            let used = total.saturating_sub(avail);
            let pct = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            DiskEntry {
                mount: d.mount_point().to_string_lossy().to_string(),
                used,
                total,
                pct,
                kind: d.kind(),
                fs: d.file_system().to_string_lossy().to_string(),
            }
        })
        .collect()
}

fn collect_sys_stats(sys: &System) -> SysStats {
    let load = System::load_average();
    SysStats {
        hostname: System::host_name().unwrap_or_default(),
        load_avg: (load.one, load.five, load.fifteen),
        mem_used: sys.used_memory(),
        mem_total: sys.total_memory(),
        cpu_count: sys.cpus().len(),
        process_count: sys.processes().len(),
        swap_used: sys.used_swap(),
        swap_total: sys.total_swap(),
        kernel: System::kernel_version().unwrap_or_default(),
        arch: System::cpu_arch().unwrap_or_default(),
        uptime: System::uptime(),
        os_name: System::name().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
    }
}

fn spawn_bg_collector(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>) {
    std::thread::spawn(move || {
        let mut sys = System::new_all();
        let mut disks = Disks::new_with_refreshed_list();
        loop {
            std::thread::sleep(Duration::from_secs(3));
            sys.refresh_all();
            disks.refresh_list();
            let stats = collect_sys_stats(&sys);
            let entries = collect_disk_entries(&disks);
            {
                let mut lock = shared.lock().unwrap();
                *lock = (stats, entries);
            }
        }
    });
}

// ─── Main ──────────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    // Initial data
    let sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    let initial_stats = collect_sys_stats(&sys);
    let initial_disks = collect_disk_entries(&disks);
    drop(sys);
    drop(disks);

    let shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>> =
        Arc::new(Mutex::new((initial_stats, initial_disks)));

    spawn_bg_collector(Arc::clone(&shared));

    let mut terminal = ratatui::init();
    let mut app = App::new(Arc::clone(&shared));
    let result = run_app(&mut terminal, &mut app);
    ratatui::restore();
    app.save();
    result
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    let mut last_data_refresh = Instant::now();

    loop {
        let refresh_dur = Duration::from_secs(app.prefs.refresh_rate);
        if last_data_refresh.elapsed() >= refresh_dur {
            app.refresh_data();
            last_data_refresh = Instant::now();
        }

        terminal.draw(|f| draw(f, app))?;

        // Poll 200ms for responsive clock
        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        app.handle_key(key);
                        if app.quit {
                            return Ok(());
                        }
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }
}
