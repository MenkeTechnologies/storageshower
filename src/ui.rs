use ratatui::{
    buffer::Buffer,
    style::{Color, Modifier, Style},
    Frame,
};
use std::time::Duration;

use crate::app::{mount_col_width, right_col_width, App};
use crate::helpers::{format_bytes, format_latency, format_rate, format_uptime, truncate_mount};
use crate::system::{chrono_now, get_battery, get_local_ip, get_tty, get_username};
use crate::types::*;

// ─── Color/style helpers ───────────────────────────────────────────────────

pub fn palette(mode: ColorMode) -> (Color, Color, Color, Color, Color, Color) {
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
        ColorMode::Amber => (
            Color::Indexed(172),
            Color::Indexed(214),
            Color::Indexed(178),
            Color::Indexed(220),
            Color::Indexed(166),
            Color::Indexed(130),
        ),
        ColorMode::Cyan => (
            Color::Indexed(37),
            Color::Indexed(50),
            Color::Indexed(44),
            Color::Indexed(87),
            Color::Indexed(30),
            Color::Indexed(23),
        ),
        ColorMode::Red => (
            Color::Indexed(160),
            Color::Indexed(203),
            Color::Indexed(196),
            Color::Indexed(210),
            Color::Indexed(124),
            Color::Indexed(88),
        ),
        ColorMode::Sakura => (
            Color::Indexed(175),
            Color::Indexed(218),
            Color::Indexed(182),
            Color::Indexed(225),
            Color::Indexed(169),
            Color::Indexed(132),
        ),
        ColorMode::Matrix => (
            Color::Indexed(22),
            Color::Indexed(46),
            Color::Indexed(28),
            Color::Indexed(119),
            Color::Indexed(34),
            Color::Indexed(22),
        ),
        ColorMode::Sunset => (
            Color::Indexed(202),
            Color::Indexed(220),
            Color::Indexed(196),
            Color::Indexed(213),
            Color::Indexed(160),
            Color::Indexed(125),
        ),
    }
}

pub fn palette_for_prefs(prefs: &crate::prefs::Prefs) -> (Color, Color, Color, Color, Color, Color) {
    if let Some(ref name) = prefs.active_theme {
        if let Some(theme) = prefs.custom_themes.get(name) {
            return (
                Color::Indexed(theme.blue),
                Color::Indexed(theme.green),
                Color::Indexed(theme.purple),
                Color::Indexed(theme.light_purple),
                Color::Indexed(theme.royal),
                Color::Indexed(theme.dark_purple),
            );
        }
    }
    palette(prefs.color_mode)
}

fn is_alert_flashing(app: &App) -> bool {
    app.alert_flash
        .map(|t| t.elapsed().as_millis() < 2000 && (t.elapsed().as_millis() / 300) % 2 == 0)
        .unwrap_or(false)
}

fn border_color(app: &App) -> Color {
    if is_alert_flashing(app) {
        return Color::Indexed(196); // red flash
    }
    let (blue, ..) = palette_for_prefs(&app.prefs);
    if app.paused { DIM_BORDER } else { blue }
}

fn thresh_color(pct: f64, app: &App) -> (Color, Option<Color>, &'static str) {
    let (_, green, _, lpurple, royal, _) = palette_for_prefs(&app.prefs);
    if pct >= app.prefs.thresh_crit as f64 {
        (royal, Some(royal), "\u{2716}")
    } else if pct >= app.prefs.thresh_warn as f64 {
        (lpurple, Some(lpurple), "\u{26A0}")
    } else {
        (green, None, "\u{25C8}")
    }
}

pub fn gradient_color_at(frac: f64, mode: ColorMode) -> Color {
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

fn gradient_color_at_prefs(frac: f64, prefs: &crate::prefs::Prefs) -> Color {
    let (blue, green, purple, _, _, dpurple) = palette_for_prefs(prefs);
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

// ─── Main draw ─────────────────────────────────────────────────────────────

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let w = area.width;
    let h = area.height;
    if w < 40 || h < 10 {
        let buf = frame.buffer_mut();
        set_str(buf, 0, 0, "Terminal too small (need 40x10)", Style::default().fg(Color::Red), w);
        return;
    }

    if app.view_mode == ViewMode::DrillDown {
        draw_drilldown(frame, app);
        return;
    }

    let buf = frame.buffer_mut();
    let bc = border_color(app);
    let border_s = Style::default().fg(bc);
    let (pal_blue, pal_green, _pal_purple, pal_lpurple, _pal_royal, _pal_dpurple) =
        palette_for_prefs(&app.prefs);

    let show_border = app.prefs.show_border;
    let lm: u16 = if show_border { 1 } else { 0 };
    let rm: u16 = if show_border { 1 } else { 0 };
    let inner_w = w.saturating_sub(lm + rm);
    let right_w = right_col_width(app);
    let pct_w: u16 = if app.prefs.col_pct_w > 0 { app.prefs.col_pct_w } else { 5 };

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
        set_cell(buf, 0, 0, "\u{2554}", border_s);
        for x in 1..w - 1 {
            set_cell(buf, x, 0, "\u{2550}", border_s);
        }
        set_cell(buf, w - 1, 0, "\u{2557}", border_s);
    }

    // Bottom border
    if show_border {
        set_cell(buf, 0, h - 1, "\u{255A}", border_s);
        for x in 1..w - 1 {
            set_cell(buf, x, h - 1, "\u{2550}", border_s);
        }
        set_cell(buf, w - 1, h - 1, "\u{255D}", border_s);
    }

    // Side borders
    if show_border {
        for y in 1..h - 1 {
            set_cell(buf, 0, y, "\u{2551}", border_s);
            set_cell(buf, w - 1, y, "\u{2551}", border_s);
        }
    }

    let mut row: u16 = if show_border { 1 } else { 0 };

    // ─── Title banner ───
    {
        let banner_s = Style::default().fg(pal_green).bg(DARK_BG);
        let accent_s = Style::default().fg(pal_blue).bg(DARK_BG).add_modifier(Modifier::BOLD);

        for x in lm..w.saturating_sub(rm) {
            set_cell(buf, x, row, " ", Style::default().bg(DARK_BG));
        }

        let now = chrono_now();
        let hostname = &app.stats.hostname;
        let s = " \u{2502} ";
        let mut title = format!(
            " \u{25B6}\u{25B6}\u{25B6} DISK MATRIX \u{25C0}\u{25C0}\u{25C0}{s}node:{}{s}date:{}{s}clock:{}",
            hostname, now.0, now.1
        );
        if app.paused {
            title.push_str(&format!("{s}\u{23F8} PAUSED"));
        }
        if app.prefs.show_local {
            title.push_str(&format!("{s}LOCAL"));
        }
        if app.prefs.show_all {
            title.push_str(&format!("{s}ALL"));
        }
        if !app.filter.is_empty() {
            title.push_str(&format!("{s}filter:{}", app.filter));
        }

        let load = app.stats.load_avg;
        let mem_pct = if app.stats.mem_total > 0 {
            (app.stats.mem_used as f64 / app.stats.mem_total as f64) * 100.0
        } else {
            0.0
        };
        title.push_str(&format!(
            "{s}load:{:.2}/{:.2}/{:.2}{s}mem:{}/{}({:.0}%){s}cpu:{}",
            load.0, load.1, load.2,
            format_bytes(app.stats.mem_used, UnitMode::Human),
            format_bytes(app.stats.mem_total, UnitMode::Human),
            mem_pct,
            app.stats.cpu_count
        ));

        if inner_w > 100 {
            title.push_str(&format!("{s}procs:{}", app.stats.process_count));
        }
        if inner_w > 120 {
            title.push_str(&format!(
                "{s}swap:{}/{}",
                format_bytes(app.stats.swap_used, UnitMode::Human),
                format_bytes(app.stats.swap_total, UnitMode::Human)
            ));
        }
        if inner_w > 140 && !app.stats.kernel.is_empty() {
            title.push_str(&format!("{s}kern:{}", app.stats.kernel));
        }
        if inner_w > 160 && !app.stats.arch.is_empty() {
            title.push_str(&format!("{s}arch:{}", app.stats.arch));
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

        if let Some(idx) = title_display.find("DISK MATRIX") {
            let char_offset = title_display[..idx].chars().count() as u16;
            set_str(buf, lm + char_offset, row, "DISK MATRIX", accent_s, 11);
        }

        row += 1;
    }

    // ─── Header separator ───
    draw_separator(buf, row, w, show_border, border_s);
    row += 1;

    // Pre-compute disk list and column widths for tabular alignment
    let disks = app.sorted_disks();
    let disk_count = disks.len();
    let (max_used_w, max_total_w) = if app.prefs.show_used {
        let mut mu = 4usize;
        let mut mt = 4usize;
        for d in &disks {
            mu = mu.max(format_bytes(d.used, app.prefs.unit_mode).len());
            mt = mt.max(format_bytes(d.total, app.prefs.unit_mode).len());
        }
        (mu, mt)
    } else {
        (0, 0)
    };

    // ─── Column headers ───
    if app.prefs.show_header {
        let hdr_s = Style::default().fg(pal_lpurple).add_modifier(Modifier::BOLD);

        for x in lm..w.saturating_sub(rm) {
            set_cell(buf, x, row, " ", Style::default());
        }

        let mount_w = mount_col_width(inner_w, &app.prefs);
        let sort_arrow = if app.prefs.sort_rev { "\u{25BC}" } else { "\u{25B2}" };

        let name_arrow = if app.prefs.sort_mode == SortMode::Name { sort_arrow } else { " " };
        let pct_arrow = if app.prefs.sort_mode == SortMode::Pct { sort_arrow } else { " " };
        let size_arrow = if app.prefs.sort_mode == SortMode::Size { sort_arrow } else { " " };

        let mount_hdr = format!(" MOUNT{}", name_arrow);
        set_str(buf, lm, row, &mount_hdr, hdr_s, (mount_w + 3) as u16);

        let bar_col_start = lm + 3 + mount_w as u16;
        set_cell(buf, bar_col_start, row, "\u{2502}", border_s);

        if app.prefs.show_bars {
            let bar_start = bar_col_start + 1;
            set_str(buf, bar_start, row, "USAGE", hdr_s, 5);

            let bar_end = w.saturating_sub(rm + right_w + 1);
            if bar_end < w.saturating_sub(rm) {
                set_cell(buf, bar_end, row, "\u{2502}", border_s);
            }
        }

        {
            let right_start = w.saturating_sub(rm + right_w);
            let pct_hdr = format!("PCT{}", pct_arrow);
            set_str(buf, right_start, row, &pct_hdr, hdr_s, pct_w);
            if app.prefs.show_used {
                let pct_sep_x = right_start + pct_w;
                set_cell(buf, pct_sep_x, row, "\u{2502}", border_s);
                let used_hdr = format!(
                    " {:>uw$}/{:>tw$}{}",
                    "USED", "SIZE", size_arrow,
                    uw = max_used_w, tw = max_total_w
                );
                let remaining = right_w.saturating_sub(pct_w + 1);
                set_str(buf, pct_sep_x + 1, row, &used_hdr, hdr_s, remaining);
            }
        }

        row += 1;
        draw_separator(buf, row, w, show_border, border_s);
        row += 1;
    }

    // ─── Footer area ───
    let footer_rows: u16 = 2 + (if show_border { 1 } else { 0 });
    let disk_area_end = h.saturating_sub(footer_rows);

    // ─── Disk rows ───
    let mount_w = mount_col_width(inner_w, &app.prefs);

    for (di, disk) in disks.iter().enumerate() {
        if row >= disk_area_end {
            break;
        }

        let is_selected = app.selected == Some(di);
        let (fg_color, bg_pct, icon) = thresh_color(disk.pct, app);
        let is_alert_row = is_alert_flashing(app) && app.alert_mounts.contains(&disk.mount);

        if is_alert_row {
            let flash_bg = Style::default().bg(Color::Indexed(52)); // dark red flash
            for x in lm..w.saturating_sub(rm) {
                set_cell(buf, x, row, " ", flash_bg);
            }
        } else if is_selected {
            let sel_bg = Style::default().bg(Color::Indexed(237));
            for x in lm..w.saturating_sub(rm) {
                set_cell(buf, x, row, " ", sel_bg);
            }
        }

        let is_bookmarked = app.prefs.bookmarks.contains(&disk.mount);
        let icon_str = if is_selected {
            format!("\u{25B8}{} ", icon)
        } else if is_bookmarked {
            format!("\u{2605}{} ", icon)
        } else {
            format!(" {} ", icon)
        };
        let icon_style = if is_selected {
            Style::default().fg(fg_color).bg(Color::Indexed(237))
        } else if is_bookmarked {
            Style::default().fg(Color::Indexed(220)) // gold star
        } else {
            Style::default().fg(fg_color)
        };
        set_str(buf, lm, row, &icon_str, icon_style, 3);

        let mount_display = if app.prefs.full_mount {
            format!("{:<width$}", disk.mount, width = mount_w.saturating_sub(1))
        } else {
            truncate_mount(&disk.mount, mount_w.saturating_sub(1))
        };
        set_str(buf, lm + 3, row, &mount_display, Style::default().fg(pal_green), mount_w as u16);

        // SMART health indicator (end of mount column)
        if let Some(smart) = disk.smart_status {
            if smart != SmartHealth::Unknown {
                let (smart_icon, smart_color) = match smart {
                    SmartHealth::Verified => ("\u{2714}", Color::Indexed(48)),
                    SmartHealth::Failing => ("\u{2718}", Color::Indexed(196)),
                    SmartHealth::Unknown => unreachable!(),
                };
                let smart_x = lm + 3 + mount_w as u16 - 2;
                set_cell(buf, smart_x, row, smart_icon, Style::default().fg(smart_color).add_modifier(Modifier::DIM));
            }
        }

        if let Some(lat) = disk.latency_ms {
            let badge = format_latency(lat);
            let lat_color = if lat < 50.0 {
                pal_green
            } else if lat < 200.0 {
                pal_lpurple
            } else {
                Color::Indexed(196)
            };
            let badge_len = badge.len() as u16;
            let badge_x = lm + 3 + mount_w as u16 - badge_len - 1;
            set_str(buf, badge_x, row, &badge, Style::default().fg(lat_color).add_modifier(Modifier::DIM), badge_len);
        }

        let bar_col_start = lm + 3 + mount_w as u16;
        set_cell(buf, bar_col_start, row, "\u{2502}", border_s);

        if app.prefs.show_bars {
            let bar_end = w.saturating_sub(rm + right_w + 1);
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
                                let gc = gradient_color_at_prefs(frac, &app.prefs);
                                let ch = if j == filled - 1 {
                                    "\u{25B8}"
                                } else if frac < 0.33 {
                                    "\u{2588}"
                                } else if frac < 0.55 {
                                    "\u{2593}"
                                } else if frac < 0.80 {
                                    "\u{2592}"
                                } else {
                                    "\u{2591}"
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
                                    set_cell(buf, x, row, "\u{25AC}", Style::default().fg(fg_color));
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
                            BarStyle::Thin => ("\u{00B7}", Color::Indexed(240)),
                            BarStyle::Ascii => ("-", Color::Indexed(240)),
                        };
                        set_cell(buf, x, row, ch, Style::default().fg(color));
                    }
                }

                if bar_end < w.saturating_sub(rm) {
                    set_cell(buf, bar_end, row, "\u{2502}", border_s);
                }

                // I/O rate overlay on bar
                let has_io = disk.io_read_rate.is_some_and(|r| r > 0.0)
                    || disk.io_write_rate.is_some_and(|w| w > 0.0);
                if has_io && bar_w > 20 {
                    let rd = disk.io_read_rate.unwrap_or(0.0);
                    let wr = disk.io_write_rate.unwrap_or(0.0);
                    let io_str = if rd > 0.0 && wr > 0.0 {
                        format!("\u{25B2}{} \u{25BC}{}", format_rate(rd), format_rate(wr))
                    } else if rd > 0.0 {
                        format!("\u{25B2}{}", format_rate(rd))
                    } else {
                        format!("\u{25BC}{}", format_rate(wr))
                    };
                    let io_len = io_str.chars().count() as u16;
                    let io_x = bar_end.saturating_sub(io_len + 1);
                    if io_x > bar_start {
                        let io_style = Style::default()
                            .fg(Color::Indexed(248))
                            .add_modifier(Modifier::DIM);
                        set_str(buf, io_x, row, &io_str, io_style, io_len);
                    }
                }
            }
        }

        let pct_str = format!("{:>3.0}%", disk.pct);
        let pct_style = if let Some(bg) = bg_pct {
            Style::default().fg(Color::White).bg(bg)
        } else {
            Style::default().fg(pal_green)
        };

        {
            let right_start = w.saturating_sub(rm + right_w);
            set_str(buf, right_start, row, &pct_str, pct_style, pct_w);
            if app.prefs.show_used {
                let pct_sep_x = right_start + pct_w;
                set_cell(buf, pct_sep_x, row, "\u{2502}", border_s);
                let used_s = format!("{:>width$}", format_bytes(disk.used, app.prefs.unit_mode), width = max_used_w);
                let total_s = format!("{:>width$}", format_bytes(disk.total, app.prefs.unit_mode), width = max_total_w);
                let size_str = format!(" {}/{}", used_s, total_s);
                let remaining = right_w.saturating_sub(pct_w + 1);
                set_str(buf, pct_sep_x + 1, row, &size_str, Style::default().fg(pal_lpurple), remaining);
            }
        }

        row += 1;
    }

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
            let color_name = if let Some(ref name) = app.prefs.active_theme {
                name.as_str()
            } else {
                app.prefs.color_mode.name()
            };
            let unit_name = match app.prefs.unit_mode {
                UnitMode::Human => "human",
                UnitMode::GiB => "GiB",
                UnitMode::MiB => "MiB",
                UnitMode::Bytes => "bytes",
            };

            let mut footer = format!(
                " \u{27E6}zpwr\u{22B7}cyberdeck\u{27E7} \u{25C0}\u{25C0}\u{25C0} vol:{} \u{2502} sort:{}{} \u{2502} {}s \u{2502} {} \u{2502} {} \u{2502} {}",
                disk_count, sort_name, sort_dir, app.prefs.refresh_rate, bar_name, color_name, unit_name
            );

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
                if let Some(tty) = get_tty() {
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
            if let Some((ref msg, t)) = app.status_msg {
                if t.elapsed() < Duration::from_secs(3) {
                    footer.push_str(&format!(" \u{2502} {}", msg));
                }
            }

            let footer_display: String = footer.chars().take(inner_w as usize).collect();
            set_str(buf, lm, frow, &footer_display, footer_s, inner_w);
        }
    }

    // ─── Filter popup ───
    if app.filter_mode {
        draw_filter_popup(buf, w, h, app);
    }

    // ─── Help overlay ───
    if app.show_help {
        draw_help(buf, w, h, app);
    }

    // ─── Theme editor overlay ───
    if app.theme_editor {
        draw_theme_editor(buf, w, h, app);
    }
}

// ─── Sub-draw functions ────────────────────────────────────────────────────

fn draw_separator(buf: &mut Buffer, row: u16, w: u16, show_border: bool, border_s: Style) {
    if show_border {
        set_cell(buf, 0, row, "\u{2560}", border_s);
        for x in 1..w - 1 {
            set_cell(buf, x, row, "\u{2550}", border_s);
        }
        set_cell(buf, w - 1, row, "\u{2563}", border_s);
    } else {
        for x in 0..w {
            set_cell(buf, x, row, "\u{2550}", border_s);
        }
    }
}

fn draw_filter_popup(buf: &mut Buffer, w: u16, h: u16, app: &App) {
    let box_w: u16 = 54u16.min(w.saturating_sub(4));
    let box_h: u16 = 9;
    let x0 = (w.saturating_sub(box_w)) / 2;
    let y0 = (h.saturating_sub(box_h)) / 2;
    let bc = border_color(app);
    let border_s = Style::default().fg(bc);
    let bg_s = Style::default().fg(Color::White).bg(HELP_BG);
    let title_s = Style::default()
        .fg(Color::Indexed(27))
        .bg(HELP_BG)
        .add_modifier(Modifier::BOLD);
    let input_s = Style::default()
        .fg(Color::Indexed(48))
        .bg(Color::Indexed(235));
    let hint_s = Style::default()
        .fg(Color::Indexed(240))
        .bg(HELP_BG);
    let label_s = Style::default()
        .fg(Color::Indexed(141))
        .bg(HELP_BG);

    for y in y0..y0 + box_h {
        for x in x0..x0 + box_w {
            set_cell(buf, x, y, " ", Style::default().bg(HELP_BG));
        }
    }

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

    let title = "\u{26A1} FILTER MOUNTS";
    let tlen = title.chars().count() as u16;
    let tx = x0 + (box_w.saturating_sub(tlen)) / 2;
    set_str(buf, tx, y0 + 1, title, title_s, box_w - 2);

    let current_label = "Active: ";
    let current_val = if app.filter.is_empty() { "(none)" } else { &app.filter };
    set_str(buf, x0 + 2, y0 + 2, current_label, label_s, 8);
    set_str(buf, x0 + 10, y0 + 2, current_val, bg_s, box_w.saturating_sub(13));

    let input_w = box_w.saturating_sub(4);
    let field_y = y0 + 3;
    for x in x0 + 2..x0 + 2 + input_w {
        set_cell(buf, x, field_y, " ", input_s);
    }
    set_str(buf, x0 + 2, field_y, "\u{25B8} ", input_s, 2);

    let max_visible = (input_w as usize).saturating_sub(3);
    let cursor_pos = app.filter_cursor;
    let buf_len = app.filter_buf.len();

    let (vis_start, vis_end) = if buf_len <= max_visible {
        (0, buf_len)
    } else if cursor_pos <= max_visible / 2 {
        (0, max_visible)
    } else if cursor_pos >= buf_len.saturating_sub(max_visible / 2) {
        (buf_len.saturating_sub(max_visible), buf_len)
    } else {
        (cursor_pos - max_visible / 2, cursor_pos + max_visible / 2)
    };

    let display_buf = &app.filter_buf[vis_start..vis_end.min(buf_len)];
    set_str(buf, x0 + 4, field_y, display_buf, input_s, input_w.saturating_sub(3));

    let cursor_x = x0 + 4 + (cursor_pos - vis_start) as u16;
    if cursor_x < x0 + 2 + input_w {
        let ch = app.filter_buf.chars().nth(cursor_pos).unwrap_or(' ');
        let cursor_s = Style::default().fg(Color::Indexed(235)).bg(Color::Indexed(48));
        set_cell(buf, cursor_x, field_y, &ch.to_string(), cursor_s);
    }

    let hints1 = "Enter=apply  Esc=cancel  ^W=del word";
    let h1x = x0 + (box_w.saturating_sub(hints1.len() as u16)) / 2;
    set_str(buf, h1x, y0 + 5, hints1, hint_s, box_w.saturating_sub(2));

    let hints2 = "^A=home ^E=end ^B/^F=\u{2190}/\u{2192} ^U=clear ^K=kill";
    let h2x = x0 + (box_w.saturating_sub(hints2.chars().count() as u16)) / 2;
    set_str(buf, h2x, y0 + 6, hints2, hint_s, box_w.saturating_sub(2));

    let hints3 = "\u{2190}/\u{2192}=move cursor  Del=delete forward";
    let h3x = x0 + (box_w.saturating_sub(hints3.chars().count() as u16)) / 2;
    set_str(buf, h3x, y0 + 7, hints3, hint_s, box_w.saturating_sub(2));
}

fn draw_drilldown(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let w = area.width;
    let h = area.height;
    let buf = frame.buffer_mut();

    let bc = border_color(app);
    let border_s = Style::default().fg(bc);
    let (pal_blue, pal_green, _pal_purple, pal_lpurple, _pal_royal, _pal_dpurple) =
        palette_for_prefs(&app.prefs);

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

    // Borders
    if show_border {
        set_cell(buf, 0, 0, "\u{2554}", border_s);
        for x in 1..w - 1 { set_cell(buf, x, 0, "\u{2550}", border_s); }
        set_cell(buf, w - 1, 0, "\u{2557}", border_s);
        set_cell(buf, 0, h - 1, "\u{255A}", border_s);
        for x in 1..w - 1 { set_cell(buf, x, h - 1, "\u{2550}", border_s); }
        set_cell(buf, w - 1, h - 1, "\u{255D}", border_s);
        for y in 1..h - 1 {
            set_cell(buf, 0, y, "\u{2551}", border_s);
            set_cell(buf, w - 1, y, "\u{2551}", border_s);
        }
    }

    let mut row: u16 = if show_border { 1 } else { 0 };

    // ─── Breadcrumb bar ───
    {
        let banner_s = Style::default().fg(pal_green).bg(DARK_BG);
        let accent_s = Style::default().fg(pal_blue).bg(DARK_BG).add_modifier(Modifier::BOLD);

        for x in lm..w.saturating_sub(rm) {
            set_cell(buf, x, row, " ", Style::default().bg(DARK_BG));
        }

        set_str(buf, lm, row, " \u{25B6} DRILL DOWN \u{25C0} ", accent_s, inner_w);
        let path_start = lm + 19;
        let path_display = app.drill_current_path();
        let remaining = inner_w.saturating_sub(19);
        set_str(buf, path_start, row, &path_display, banner_s, remaining);
        row += 1;
    }

    // ─── Separator ───
    draw_separator(buf, row, w, show_border, border_s);
    row += 1;

    // ─── Column header ───
    {
        let hdr_s = Style::default().fg(pal_lpurple).add_modifier(Modifier::BOLD);
        let sort_arrow = if app.drill_sort_rev { "\u{25BC}" } else { "\u{25B2}" };
        let name_arrow = if app.drill_sort == DrillSortMode::Name { sort_arrow } else { " " };
        let size_arrow = if app.drill_sort == DrillSortMode::Size { sort_arrow } else { " " };
        let hdr = format!("   {}{:<name_w$} {:>9}{}", name_arrow, "NAME", "SIZE", size_arrow, name_w = (inner_w as usize).saturating_sub(16));
        set_str(buf, lm, row, &hdr, hdr_s, inner_w);
        row += 1;
        draw_separator(buf, row, w, show_border, border_s);
        row += 1;
    }

    // ─── Footer area ───
    let footer_rows: u16 = 2 + (if show_border { 1 } else { 0 });
    let entry_area_end = h.saturating_sub(footer_rows);

    // ─── Scanning indicator ───
    if app.drill_scanning {
        let scanning_s = Style::default().fg(pal_blue).add_modifier(Modifier::BOLD);
        set_str(buf, lm + 2, row, "\u{25CB} Scanning\u{2026}", scanning_s, inner_w);
        row += 1;
    }

    // ─── Entries ───
    let max_size = app.drill_entries.first().map(|e| e.size).unwrap_or(1).max(1);

    for (i, entry) in app.drill_entries.iter().enumerate() {
        if row >= entry_area_end {
            break;
        }

        let is_selected = i == app.drill_selected;

        if is_selected {
            let sel_bg = Style::default().bg(Color::Indexed(237));
            for x in lm..w.saturating_sub(rm) {
                set_cell(buf, x, row, " ", sel_bg);
            }
        }

        // Icon
        let (icon, icon_color) = if entry.is_dir {
            ("\u{25B8} \u{1F4C1} ", pal_blue)
        } else {
            ("  \u{25CB} ", pal_green)
        };
        let icon_style = if is_selected {
            Style::default().fg(icon_color).bg(Color::Indexed(237))
        } else {
            Style::default().fg(icon_color)
        };
        set_str(buf, lm, row, icon, icon_style, 5);

        // Name
        let size_col_w = 10u16;
        let bar_col_w = 12u16;
        let name_max = (inner_w as usize).saturating_sub(5 + size_col_w as usize + bar_col_w as usize + 2);
        let name_display = truncate_mount(&entry.name, name_max);
        let name_style = if is_selected {
            Style::default().fg(pal_green).bg(Color::Indexed(237))
        } else {
            Style::default().fg(pal_green)
        };
        set_str(buf, lm + 5, row, &name_display, name_style, name_max as u16);

        // Size bar
        let bar_start = lm + 5 + name_max as u16 + 1;
        let bar_w = bar_col_w as usize;
        let frac = entry.size as f64 / max_size as f64;
        let filled = (frac * bar_w as f64).round() as usize;
        for j in 0..bar_w {
            let x = bar_start + j as u16;
            if j < filled {
                let gc = gradient_color_at_prefs(j as f64 / bar_w as f64, &app.prefs);
                set_cell(buf, x, row, "\u{2588}", Style::default().fg(gc));
            } else {
                set_cell(buf, x, row, "\u{00B7}", Style::default().fg(DIM_BORDER));
            }
        }

        // Size text
        let size_str = format_bytes(entry.size, app.prefs.unit_mode);
        let size_display = format!("{:>10}", size_str);
        let size_style = if is_selected {
            Style::default().fg(pal_lpurple).bg(Color::Indexed(237))
        } else {
            Style::default().fg(pal_lpurple)
        };
        let size_x = w.saturating_sub(rm + size_col_w);
        set_str(buf, size_x, row, &size_display, size_style, size_col_w);

        row += 1;
    }

    // ─── Empty state ───
    if app.drill_entries.is_empty() && !app.drill_scanning {
        let empty_s = Style::default().fg(DIM_BORDER);
        set_str(buf, lm + 2, row, "(empty or access denied)", empty_s, inner_w);
    }

    // ─── Footer separator ───
    if entry_area_end < h {
        draw_separator(buf, entry_area_end, w, show_border, border_s);
    }

    // ─── Footer banner ───
    {
        let frow = entry_area_end + 1;
        if frow < h {
            let footer_s = Style::default().fg(pal_green).bg(DARK_BG);
            for x in lm..w.saturating_sub(rm) {
                set_cell(buf, x, frow, " ", Style::default().bg(DARK_BG));
            }

            let entry_count = app.drill_entries.len();
            let total_size: u64 = app.drill_entries.iter().map(|e| e.size).sum();
            let sort_name = match app.drill_sort {
                DrillSortMode::Size => "size",
                DrillSortMode::Name => "name",
            };
            let sort_dir = if app.drill_sort_rev { "\u{25BC}" } else { "\u{25B2}" };
            let footer = format!(
                " \u{27E6}drill\u{22B7}down\u{27E7} \u{25C0}\u{25C0}\u{25C0} items:{} \u{2502} total:{} \u{2502} sort:{}{} \u{2502} s:size \u{2502} n:name \u{2502} r:rev \u{2502} bksp:back",
                entry_count,
                format_bytes(total_size, app.prefs.unit_mode),
                sort_name, sort_dir,
            );
            let footer_display: String = footer.chars().take(inner_w as usize).collect();
            set_str(buf, lm, frow, &footer_display, footer_s, inner_w);
        }
    }
}

fn draw_theme_editor(buf: &mut Buffer, w: u16, h: u16, app: &App) {
    let box_w: u16 = 56u16.min(w.saturating_sub(4));
    let box_h: u16 = if app.theme_edit_naming { 16 } else { 15 };
    let x0 = (w.saturating_sub(box_w)) / 2;
    let y0 = (h.saturating_sub(box_h)) / 2;
    let bc = border_color(app);
    let border_s = Style::default().fg(bc);
    let bg_s = Style::default().fg(Color::White).bg(HELP_BG);
    let title_s = Style::default()
        .fg(Color::Indexed(27))
        .bg(HELP_BG)
        .add_modifier(Modifier::BOLD);
    let hint_s = Style::default().fg(Color::Indexed(240)).bg(HELP_BG);
    let sel_s = Style::default().fg(Color::White).bg(Color::Indexed(237));

    // Draw box
    for y in y0..y0 + box_h {
        for x in x0..x0 + box_w {
            set_cell(buf, x, y, " ", Style::default().bg(HELP_BG));
        }
    }
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
    let title = "\u{1F3A8} THEME EDITOR";
    let tlen = title.chars().count() as u16;
    let tx = x0 + (box_w.saturating_sub(tlen)) / 2;
    set_str(buf, tx, y0 + 1, title, title_s, box_w - 2);

    // Color channel names
    let labels = ["blue", "green", "purple", "light_purple", "royal", "dark_purple"];
    let colors = app.theme_edit_colors;

    for (i, label) in labels.iter().enumerate() {
        let row_y = y0 + 3 + i as u16;
        let is_sel = i == app.theme_edit_slot;

        // Selection indicator
        let row_style = if is_sel { sel_s } else { bg_s };
        if is_sel {
            for x in x0 + 1..x0 + box_w - 1 {
                set_cell(buf, x, row_y, " ", sel_s);
            }
        }

        let marker = if is_sel { "\u{25B8} " } else { "  " };
        set_str(buf, x0 + 2, row_y, marker, row_style, 2);

        // Label
        let label_str = format!("{:<14}", label);
        set_str(buf, x0 + 4, row_y, &label_str, row_style, 14);

        // Value
        let val_str = format!("{:>3}", colors[i]);
        set_str(buf, x0 + 19, row_y, &val_str, row_style, 3);

        // Color swatch — two blocks showing the color
        let swatch_style = Style::default().fg(Color::Indexed(colors[i])).bg(HELP_BG);
        set_str(buf, x0 + 24, row_y, "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}", swatch_style, 5);

        // Gradient preview bar using all 6 current colors as a mini bar
        let preview_color = Color::Indexed(colors[i]);
        set_str(buf, x0 + 30, row_y, " \u{25C0}\u{2500}\u{2500}\u{25B6}", Style::default().fg(preview_color).bg(HELP_BG), 5);
    }

    // Preview bar using the full palette
    let preview_y = y0 + 10;
    set_str(buf, x0 + 2, preview_y, "preview:", hint_s, 8);
    let preview_w = (box_w as usize).saturating_sub(13);
    for j in 0..preview_w {
        let frac = j as f64 / preview_w as f64;
        let c = if frac < 0.33 {
            Color::Indexed(colors[1]) // green
        } else if frac < 0.55 {
            Color::Indexed(colors[0]) // blue
        } else if frac < 0.80 {
            Color::Indexed(colors[2]) // purple
        } else {
            Color::Indexed(colors[5]) // dark_purple
        };
        set_cell(buf, x0 + 11 + j as u16, preview_y, "\u{2588}", Style::default().fg(c).bg(HELP_BG));
    }

    // Naming prompt or keybind hints
    if app.theme_edit_naming {
        let name_y = y0 + 12;
        let input_s = Style::default().fg(Color::Indexed(48)).bg(Color::Indexed(235));
        set_str(buf, x0 + 2, name_y, "Theme name:", bg_s, 11);
        let name_display = format!("{}_", app.theme_edit_name);
        set_str(buf, x0 + 14, name_y, &name_display, input_s, box_w - 16);
        set_str(buf, x0 + 2, name_y + 1, "Enter:save  Esc:back", hint_s, box_w - 4);
    } else {
        let hint_y = y0 + 12;
        set_str(buf, x0 + 2, hint_y, "j/k:select  h/l:\u{00B1}1  H/L:\u{00B1}10", hint_s, box_w - 4);
        set_str(buf, x0 + 2, hint_y + 1, "Enter/s:save  Esc/q:cancel", hint_s, box_w - 4);
    }
}

fn draw_help(buf: &mut Buffer, w: u16, h: u16, app: &App) {
    let box_w: u16 = 100u16.min(w.saturating_sub(4));
    let box_h: u16 = 46u16.min(h.saturating_sub(4));
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

    for y in y0..y0 + box_h {
        for x in x0..x0 + box_w {
            set_cell(buf, x, y, " ", Style::default().bg(HELP_BG));
        }
    }

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

    let title = "\u{2328} DISK MATRIX \u{2014} KEYBOARD SHORTCUTS";
    let tlen = title.chars().count() as u16;
    let tx = x0 + (box_w.saturating_sub(tlen)) / 2;
    set_str(buf, tx, y0 + 1, title, title_s, box_w - 2);

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
        HelpEntry { key: "a/A", desc: "All filesystems", val_fn: |a| format!("[{}]", if a.prefs.show_all {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "SORT", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "n/N", desc: "Sort by name", val_fn: |a| if a.prefs.sort_mode == SortMode::Name {"[active]".into()} else {String::new()}, is_section: false },
        HelpEntry { key: "u/U", desc: "Sort by usage %", val_fn: |a| if a.prefs.sort_mode == SortMode::Pct {"[active]".into()} else {String::new()}, is_section: false },
        HelpEntry { key: "s/S", desc: "Sort by size", val_fn: |a| if a.prefs.sort_mode == SortMode::Size {"[active]".into()} else {String::new()}, is_section: false },
        HelpEntry { key: "r/R", desc: "Reverse sort", val_fn: |a| format!("[{}]", if a.prefs.sort_rev {"\u{25BC}"} else {"\u{25B2}"}), is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "DISPLAY", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "b", desc: "Cycle bar style", val_fn: |a| format!("[{}]", match a.prefs.bar_style { BarStyle::Gradient=>"gradient", BarStyle::Solid=>"solid", BarStyle::Thin=>"thin", BarStyle::Ascii=>"ascii" }), is_section: false },
        HelpEntry { key: "c", desc: "Cycle color mode", val_fn: |a| { let n = if let Some(ref t) = a.prefs.active_theme { t.clone() } else { a.prefs.color_mode.name().into() }; format!("[{}]", n) }, is_section: false },
        HelpEntry { key: "C", desc: "Theme editor", val_fn: empty_val, is_section: false },
        HelpEntry { key: "v/V", desc: "Toggle bars", val_fn: |a| format!("[{}]", if a.prefs.show_bars {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "d/D", desc: "Toggle used/size", val_fn: |a| format!("[{}]", if a.prefs.show_used {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "g", desc: "Toggle col headers", val_fn: |a| format!("[{}]", if a.prefs.show_header {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "x/X", desc: "Toggle border", val_fn: |a| format!("[{}]", if a.prefs.show_border {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "m/M", desc: "Compact mounts", val_fn: |a| format!("[{}]", if a.prefs.compact {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "w/W", desc: "Full mount paths", val_fn: |a| format!("[{}]", if a.prefs.full_mount {"on"} else {"off"}), is_section: false },
        HelpEntry { key: "i/I", desc: "Cycle units", val_fn: |a| format!("[{}]", match a.prefs.unit_mode { UnitMode::Human=>"human", UnitMode::GiB=>"GiB", UnitMode::MiB=>"MiB", UnitMode::Bytes=>"bytes" }), is_section: false },
        HelpEntry { key: "t", desc: "Cycle warn threshold", val_fn: |a| format!("[{}%]", a.prefs.thresh_warn), is_section: false },
        HelpEntry { key: "T", desc: "Cycle crit threshold", val_fn: |a| format!("[{}%]", a.prefs.thresh_crit), is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "FILTER", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "/", desc: "Filter (vim-style edit)", val_fn: empty_val, is_section: false },
        HelpEntry { key: "0", desc: "Clear filter", val_fn: |a| if a.filter.is_empty() {String::new()} else {format!("[{}]", a.filter)}, is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "NAV", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "j/\u{2193}", desc: "Select next disk", val_fn: empty_val, is_section: false },
        HelpEntry { key: "k/\u{2191}", desc: "Select prev disk", val_fn: empty_val, is_section: false },
        HelpEntry { key: "G/End", desc: "Jump to last disk", val_fn: empty_val, is_section: false },
        HelpEntry { key: "^G/Home", desc: "Jump to first disk", val_fn: empty_val, is_section: false },
        HelpEntry { key: "^D/^U", desc: "Half-page down/up", val_fn: empty_val, is_section: false },
        HelpEntry { key: "Esc", desc: "Deselect", val_fn: empty_val, is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "ACTIONS", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "Enter", desc: "Drill down into mount", val_fn: empty_val, is_section: false },
        HelpEntry { key: "o/O", desc: "Open in file manager", val_fn: empty_val, is_section: false },
        HelpEntry { key: "y/Y", desc: "Copy mount to clipboard", val_fn: empty_val, is_section: false },
        HelpEntry { key: "e/E", desc: "Export to file", val_fn: empty_val, is_section: false },
        HelpEntry { key: "B", desc: "Toggle bookmark \u{2605}", val_fn: |a| format!("[{}]", a.prefs.bookmarks.len()), is_section: false },
        HelpEntry { key: "?", desc: "Show help", val_fn: empty_val, is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "DRILL DOWN", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "Enter", desc: "Into subdirectory", val_fn: empty_val, is_section: false },
        HelpEntry { key: "Bksp", desc: "Up one level", val_fn: empty_val, is_section: false },
        HelpEntry { key: "Esc", desc: "Back to disk list", val_fn: empty_val, is_section: false },
        HelpEntry { key: "s/n", desc: "Sort by size/name", val_fn: empty_val, is_section: false },
        HelpEntry { key: "r", desc: "Reverse sort", val_fn: empty_val, is_section: false },
        HelpEntry { key: "o/O", desc: "Open directory", val_fn: empty_val, is_section: false },
        HelpEntry { key: "", desc: "", val_fn: empty_val, is_section: false },
        HelpEntry { key: "MOUSE", desc: "", val_fn: empty_val, is_section: true },
        HelpEntry { key: "Click", desc: "Select disk row", val_fn: empty_val, is_section: false },
        HelpEntry { key: "Click\u{00D7}2", desc: "Drill into selected", val_fn: empty_val, is_section: false },
        HelpEntry { key: "Drag", desc: "Resize columns", val_fn: empty_val, is_section: false },
        HelpEntry { key: "R-Click", desc: "Toggle help", val_fn: empty_val, is_section: false },
    ];

    let content_h = (box_h as usize).saturating_sub(4);
    let half = content_h.div_ceil(2);
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn palette_returns_six_colors() {
        for &mode in ColorMode::ALL {
            let (a, b, c, d, e, f) = palette(mode);
            // Just verify they are Color::Indexed values (not default)
            assert_ne!(a, Color::Reset);
            assert_ne!(b, Color::Reset);
            assert_ne!(c, Color::Reset);
            assert_ne!(d, Color::Reset);
            assert_ne!(e, Color::Reset);
            assert_ne!(f, Color::Reset);
        }
    }

    #[test]
    fn palette_modes_differ() {
        let default = palette(ColorMode::Default);
        let green = palette(ColorMode::Green);
        // At least the first color should differ
        assert_ne!(default.0, green.0);
    }

    #[test]
    fn set_cell_within_bounds() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        set_cell(&mut buf, 3, 2, "X", Style::default());
        assert_eq!(buf[(3, 2)].symbol(), "X");
    }

    #[test]
    fn set_cell_out_of_bounds_no_panic() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        set_cell(&mut buf, 100, 100, "X", Style::default());
        // Should not panic
    }

    #[test]
    fn set_str_writes_string() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 5));
        set_str(&mut buf, 0, 0, "Hello", Style::default(), 20);
        assert_eq!(buf[(0, 0)].symbol(), "H");
        assert_eq!(buf[(1, 0)].symbol(), "e");
        assert_eq!(buf[(4, 0)].symbol(), "o");
    }

    #[test]
    fn set_str_truncates_at_max_w() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 5));
        set_str(&mut buf, 0, 0, "Hello World", Style::default(), 5);
        assert_eq!(buf[(4, 0)].symbol(), "o");
        assert_eq!(buf[(5, 0)].symbol(), " "); // not written to
    }

    #[test]
    fn gradient_color_at_returns_colors() {
        for &mode in ColorMode::ALL {
            let c0 = gradient_color_at(0.0, mode);
            let c50 = gradient_color_at(0.5, mode);
            let c90 = gradient_color_at(0.9, mode);
            assert_ne!(c0, Color::Reset);
            assert_ne!(c50, Color::Reset);
            assert_ne!(c90, Color::Reset);
        }
    }

    #[test]
    fn border_color_paused_vs_normal() {
        use std::sync::{Arc, Mutex};
        use crate::app::App;

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);

        let normal = border_color(&app);
        app.paused = true;
        let paused = border_color(&app);
        assert_ne!(normal, paused);
        assert_eq!(paused, DIM_BORDER);
    }

    #[test]
    fn thresh_color_levels() {
        use std::sync::{Arc, Mutex};
        use crate::app::App;

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let app = App::new_default(shared);

        let (_, bg_low, icon_low) = thresh_color(30.0, &app);
        assert!(bg_low.is_none());
        assert_eq!(icon_low, "\u{25C8}");

        let (_, bg_warn, icon_warn) = thresh_color(75.0, &app);
        assert!(bg_warn.is_some());
        assert_eq!(icon_warn, "\u{26A0}");

        let (_, bg_crit, icon_crit) = thresh_color(95.0, &app);
        assert!(bg_crit.is_some());
        assert_eq!(icon_crit, "\u{2716}");
    }

    // ── Gradient at boundary values ─────────────────────────

    #[test]
    fn gradient_color_at_zero() {
        for &mode in ColorMode::ALL {
            let c = gradient_color_at(0.0, mode);
            assert_ne!(c, Color::Reset);
        }
    }

    #[test]
    fn gradient_color_at_one() {
        for &mode in ColorMode::ALL {
            let c = gradient_color_at(1.0, mode);
            assert_ne!(c, Color::Reset);
        }
    }

    #[test]
    fn gradient_color_at_half() {
        for &mode in ColorMode::ALL {
            let c = gradient_color_at(0.5, mode);
            assert_ne!(c, Color::Reset);
        }
    }

    #[test]
    fn gradient_color_at_near_zero() {
        let c = gradient_color_at(0.001, ColorMode::Default);
        assert_ne!(c, Color::Reset);
    }

    #[test]
    fn gradient_color_at_near_one() {
        let c = gradient_color_at(0.999, ColorMode::Default);
        assert_ne!(c, Color::Reset);
    }

    // ── Palette all modes return 6 colors ───────────────────

    #[test]
    fn all_palettes_return_six() {
        for &mode in ColorMode::ALL {
            let (a, b, c, d, e, f) = palette(mode);
            // All should be valid non-reset colors
            for color in [a, b, c, d, e, f] {
                assert_ne!(color, Color::Reset, "palette({:?}) returned Reset", mode);
            }
        }
    }

    // ── set_cell at boundaries ──────────────────────────────

    #[test]
    fn set_cell_at_origin() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        set_cell(&mut buf, 0, 0, "A", Style::default());
        assert_eq!(buf[(0, 0)].symbol(), "A");
    }

    #[test]
    fn set_cell_at_max_valid() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        set_cell(&mut buf, 9, 4, "Z", Style::default());
        assert_eq!(buf[(9, 4)].symbol(), "Z");
    }

    #[test]
    fn set_cell_just_out_of_bounds_no_panic() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        set_cell(&mut buf, 10, 5, "X", Style::default()); // should not panic
    }

    // ── set_str edge cases ──────────────────────────────────

    #[test]
    fn set_str_empty_string() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 5));
        set_str(&mut buf, 0, 0, "", Style::default(), 20);
        assert_eq!(buf[(0, 0)].symbol(), " "); // unchanged
    }

    #[test]
    fn set_str_max_w_zero() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 5));
        set_str(&mut buf, 0, 0, "Hello", Style::default(), 0);
        assert_eq!(buf[(0, 0)].symbol(), " "); // nothing written
    }

    #[test]
    fn set_str_at_buffer_edge() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
        set_str(&mut buf, 3, 0, "Hello", Style::default(), 10);
        // Should write "He" (positions 3,4) and not panic
        assert_eq!(buf[(3, 0)].symbol(), "H");
        assert_eq!(buf[(4, 0)].symbol(), "e");
    }

    // ── thresh_color boundary values ────────────────────────

    #[test]
    fn thresh_color_at_exact_warn() {
        use std::sync::{Arc, Mutex};
        use crate::app::App;

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs.thresh_warn = 70;

        let (_, bg, icon) = thresh_color(70.0, &app);
        assert!(bg.is_some());
        assert_eq!(icon, "\u{26A0}");
    }

    #[test]
    fn thresh_color_at_exact_crit() {
        use std::sync::{Arc, Mutex};
        use crate::app::App;

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs.thresh_crit = 90;

        let (_, bg, icon) = thresh_color(90.0, &app);
        assert!(bg.is_some());
        assert_eq!(icon, "\u{2716}");
    }

    #[test]
    fn thresh_color_just_below_warn() {
        use std::sync::{Arc, Mutex};
        use crate::app::App;

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs.thresh_warn = 70;

        let (_, bg, icon) = thresh_color(69.9, &app);
        assert!(bg.is_none());
        assert_eq!(icon, "\u{25C8}");
    }

    #[test]
    fn thresh_color_at_zero() {
        use std::sync::{Arc, Mutex};
        use crate::app::App;

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let app = App::new_default(shared);

        let (_, bg, icon) = thresh_color(0.0, &app);
        assert!(bg.is_none());
        assert_eq!(icon, "\u{25C8}");
    }

    #[test]
    fn thresh_color_at_hundred() {
        use std::sync::{Arc, Mutex};
        use crate::app::App;

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let app = App::new_default(shared);

        let (_, bg, icon) = thresh_color(100.0, &app);
        assert!(bg.is_some());
        assert_eq!(icon, "\u{2716}");
    }

    // ── border_color with different color modes ─────────────

    #[test]
    fn border_color_different_modes() {
        use std::sync::{Arc, Mutex};
        use crate::app::App;

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);

        for &mode in ColorMode::ALL {
            app.prefs.color_mode = mode;
            app.paused = false;
            let c = border_color(&app);
            assert_ne!(c, Color::Reset, "border_color with {:?} returned Reset", mode);
        }
    }
}
