use ratatui::{
    Frame,
    buffer::Buffer,
    style::{Color, Modifier, Style},
};
use std::time::Duration;

use crate::app::{App, mount_col_width, right_col_width};
use crate::helpers::{format_bytes, format_latency, format_rate, format_uptime, truncate_mount};
use crate::system::{
    chrono_now, dedup_disk_totals, get_battery, get_local_ip, get_tty, get_username,
};
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
        // ── New cyberpunk palettes ──────────────────────────────
        ColorMode::NeonNoir => (
            Color::Indexed(201), // hot magenta
            Color::Indexed(231), // bright white
            Color::Indexed(93),  // deep violet
            Color::Indexed(219), // soft pink
            Color::Indexed(57),  // blue-violet
            Color::Indexed(53),  // dark plum
        ),
        ColorMode::ChromeHeart => (
            Color::Indexed(250), // silver
            Color::Indexed(255), // bright silver
            Color::Indexed(246), // mid gray
            Color::Indexed(253), // light gray
            Color::Indexed(243), // steel
            Color::Indexed(239), // gunmetal
        ),
        ColorMode::BladeRunner => (
            Color::Indexed(208), // deep orange
            Color::Indexed(37),  // muted teal
            Color::Indexed(166), // burnt orange
            Color::Indexed(73),  // dusty cyan
            Color::Indexed(130), // rust
            Color::Indexed(23),  // dark teal
        ),
        ColorMode::VoidWalker => (
            Color::Indexed(55),  // deep purple
            Color::Indexed(99),  // medium purple
            Color::Indexed(54),  // dark magenta
            Color::Indexed(141), // light lavender
            Color::Indexed(92),  // plum
            Color::Indexed(17),  // abyss blue
        ),
        ColorMode::ToxicWaste => (
            Color::Indexed(118), // lime green
            Color::Indexed(190), // yellow-green
            Color::Indexed(154), // chartreuse
            Color::Indexed(226), // acid yellow
            Color::Indexed(82),  // bright green
            Color::Indexed(58),  // olive
        ),
        ColorMode::CyberFrost => (
            Color::Indexed(159), // pale ice blue
            Color::Indexed(195), // frost white
            Color::Indexed(153), // powder blue
            Color::Indexed(189), // light periwinkle
            Color::Indexed(111), // cornflower
            Color::Indexed(67),  // slate blue
        ),
        ColorMode::PlasmaCore => (
            Color::Indexed(199), // hot pink
            Color::Indexed(213), // light pink
            Color::Indexed(163), // deep magenta
            Color::Indexed(207), // orchid
            Color::Indexed(126), // dark magenta
            Color::Indexed(89),  // purple-red
        ),
        ColorMode::SteelNerve => (
            Color::Indexed(68),  // steel blue
            Color::Indexed(110), // light steel
            Color::Indexed(60),  // slate
            Color::Indexed(146), // gray-blue
            Color::Indexed(24),  // navy steel
            Color::Indexed(236), // dark iron
        ),
        ColorMode::DarkSignal => (
            Color::Indexed(30), // dark cyan
            Color::Indexed(43), // medium spring green
            Color::Indexed(23), // deep teal
            Color::Indexed(79), // sea green
            Color::Indexed(29), // jungle green
            Color::Indexed(16), // near black
        ),
        ColorMode::GlitchPop => (
            Color::Indexed(201), // magenta
            Color::Indexed(51),  // electric cyan
            Color::Indexed(226), // yellow
            Color::Indexed(47),  // neon green
            Color::Indexed(196), // red
            Color::Indexed(21),  // blue
        ),
        ColorMode::HoloShift => (
            Color::Indexed(123), // aqua
            Color::Indexed(219), // pink
            Color::Indexed(159), // light cyan
            Color::Indexed(183), // thistle
            Color::Indexed(87),  // turquoise
            Color::Indexed(133), // medium orchid
        ),
        ColorMode::NightCity => (
            Color::Indexed(214), // warm orange
            Color::Indexed(227), // warm yellow
            Color::Indexed(209), // salmon
            Color::Indexed(223), // light salmon
            Color::Indexed(172), // dark orange
            Color::Indexed(94),  // brown
        ),
        ColorMode::DeepNet => (
            Color::Indexed(19), // navy
            Color::Indexed(33), // dodger blue
            Color::Indexed(17), // dark navy
            Color::Indexed(75), // royal blue
            Color::Indexed(26), // medium blue
            Color::Indexed(16), // near black
        ),
        ColorMode::LaserGrid => (
            Color::Indexed(46),  // neon green
            Color::Indexed(201), // neon magenta
            Color::Indexed(51),  // neon cyan
            Color::Indexed(226), // neon yellow
            Color::Indexed(196), // neon red
            Color::Indexed(21),  // neon blue
        ),
        ColorMode::QuantumFlux => (
            Color::Indexed(135), // medium purple
            Color::Indexed(75),  // steel blue
            Color::Indexed(171), // plum
            Color::Indexed(111), // cornflower
            Color::Indexed(98),  // slate purple
            Color::Indexed(61),  // dark slate blue
        ),
        ColorMode::BioHazard => (
            Color::Indexed(148), // dark khaki green
            Color::Indexed(184), // yellow-green
            Color::Indexed(106), // olive drab
            Color::Indexed(192), // light green-yellow
            Color::Indexed(64),  // dark olive
            Color::Indexed(22),  // deep green
        ),
        ColorMode::Darkwave => (
            Color::Indexed(53),  // dark plum
            Color::Indexed(140), // medium purple
            Color::Indexed(89),  // dark red-purple
            Color::Indexed(176), // light plum
            Color::Indexed(127), // magenta-purple
            Color::Indexed(52),  // dark maroon
        ),
        ColorMode::Overlock => (
            Color::Indexed(196), // red
            Color::Indexed(208), // orange
            Color::Indexed(160), // dark red
            Color::Indexed(214), // gold
            Color::Indexed(124), // firebrick
            Color::Indexed(52),  // maroon
        ),
        ColorMode::Megacorp => (
            Color::Indexed(252), // light gray
            Color::Indexed(39),  // deepskyblue
            Color::Indexed(245), // gray
            Color::Indexed(81),  // light sky blue
            Color::Indexed(242), // dark gray
            Color::Indexed(236), // charcoal
        ),
        ColorMode::Zaibatsu => (
            Color::Indexed(167), // indian red
            Color::Indexed(216), // light coral
            Color::Indexed(131), // dark red-brown
            Color::Indexed(224), // misty rose
            Color::Indexed(95),  // sienna
            Color::Indexed(52),  // dark maroon
        ),
    }
}

pub fn palette_for_prefs(
    prefs: &crate::prefs::Prefs,
) -> (Color, Color, Color, Color, Color, Color) {
    if let Some(ref name) = prefs.active_theme
        && let Some(theme) = prefs.custom_themes.get(name)
    {
        return (
            Color::Indexed(theme.blue),
            Color::Indexed(theme.green),
            Color::Indexed(theme.purple),
            Color::Indexed(theme.light_purple),
            Color::Indexed(theme.royal),
            Color::Indexed(theme.dark_purple),
        );
    }
    palette(prefs.color_mode)
}

fn is_alert_flashing(app: &App) -> bool {
    app.alert
        .flash
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
        set_str(
            buf,
            0,
            0,
            "Terminal too small (need 40x10)",
            Style::default().fg(Color::Red),
            w,
        );
        return;
    }

    if app.drill.mode == ViewMode::DrillDown {
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
    let pct_w: u16 = if app.prefs.col_pct_w > 0 {
        app.prefs.col_pct_w
    } else {
        5
    };

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
        let accent_s = Style::default()
            .fg(pal_blue)
            .bg(DARK_BG)
            .add_modifier(Modifier::BOLD);

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
        if !app.filter.text.is_empty() {
            title.push_str(&format!("{s}filter:{}", app.filter.text));
        }

        let load = app.stats.load_avg;
        let mem_pct = if app.stats.mem_total > 0 {
            (app.stats.mem_used as f64 / app.stats.mem_total as f64) * 100.0
        } else {
            0.0
        };
        title.push_str(&format!(
            "{s}load:{:.2}/{:.2}/{:.2}{s}mem:{}/{}({:.0}%){s}cpu:{}",
            load.0,
            load.1,
            load.2,
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
        for d in disks {
            mu = mu.max(format_bytes(d.used, app.prefs.unit_mode).len());
            mt = mt.max(format_bytes(d.total, app.prefs.unit_mode).len());
        }
        (mu, mt)
    } else {
        (0, 0)
    };

    // ─── Column headers ───
    if app.prefs.show_header {
        let hdr_s = Style::default()
            .fg(pal_lpurple)
            .add_modifier(Modifier::BOLD);

        for x in lm..w.saturating_sub(rm) {
            set_cell(buf, x, row, " ", Style::default());
        }

        let mount_w = mount_col_width(inner_w, &app.prefs);
        let sort_arrow = if app.prefs.sort_rev {
            "\u{25BC}"
        } else {
            "\u{25B2}"
        };

        let name_arrow = if app.prefs.sort_mode == SortMode::Name {
            sort_arrow
        } else {
            " "
        };
        let pct_arrow = if app.prefs.sort_mode == SortMode::Pct {
            sort_arrow
        } else {
            " "
        };
        let size_arrow = if app.prefs.sort_mode == SortMode::Size {
            sort_arrow
        } else {
            " "
        };

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
                    "USED",
                    "SIZE",
                    size_arrow,
                    uw = max_used_w,
                    tw = max_total_w
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

    // ─── Disk rows (with scroll offset) ───
    let mount_w = mount_col_width(inner_w, &app.prefs);

    for (di, disk) in disks.iter().enumerate().skip(app.scroll_offset) {
        if row >= disk_area_end {
            break;
        }

        let is_selected = app.selected == Some(di);
        let (fg_color, bg_pct, icon) = thresh_color(disk.pct, app);
        let is_alert_row = is_alert_flashing(app) && app.alert.mounts.contains(&disk.mount);

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
        set_str(
            buf,
            lm + 3,
            row,
            &mount_display,
            Style::default().fg(pal_green),
            mount_w as u16,
        );

        // SMART health indicator (end of mount column)
        if let Some(smart) = disk.smart_status
            && smart != SmartHealth::Unknown
        {
            let (smart_icon, smart_color) = match smart {
                SmartHealth::Verified => ("\u{2714}", Color::Indexed(48)),
                SmartHealth::Failing => ("\u{2718}", Color::Indexed(196)),
                SmartHealth::Unknown => unreachable!(),
            };
            let smart_x = lm + 3 + mount_w as u16 - 2;
            set_cell(
                buf,
                smart_x,
                row,
                smart_icon,
                Style::default().fg(smart_color).add_modifier(Modifier::DIM),
            );
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
            set_str(
                buf,
                badge_x,
                row,
                &badge,
                Style::default().fg(lat_color).add_modifier(Modifier::DIM),
                badge_len,
            );
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
                                    set_cell(
                                        buf,
                                        x,
                                        row,
                                        "\u{25B8}",
                                        Style::default().fg(fg_color),
                                    );
                                } else {
                                    set_cell(
                                        buf,
                                        x,
                                        row,
                                        "\u{25AC}",
                                        Style::default().fg(fg_color),
                                    );
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
                let used_s = format!(
                    "{:>width$}",
                    format_bytes(disk.used, app.prefs.unit_mode),
                    width = max_used_w
                );
                let total_s = format!(
                    "{:>width$}",
                    format_bytes(disk.total, app.prefs.unit_mode),
                    width = max_total_w
                );
                let size_str = format!(" {}/{}", used_s, total_s);
                let remaining = right_w.saturating_sub(pct_w + 1);
                set_str(
                    buf,
                    pct_sep_x + 1,
                    row,
                    &size_str,
                    Style::default().fg(pal_lpurple),
                    remaining,
                );
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
            let sort_dir = if app.prefs.sort_rev {
                "\u{25BC}"
            } else {
                "\u{25B2}"
            };
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
                disk_count,
                sort_name,
                sort_dir,
                app.prefs.refresh_rate,
                bar_name,
                color_name,
                unit_name
            );

            footer.push_str(&format!(" \u{2502} up:{}", format_uptime(app.stats.uptime)));

            if inner_w > 80
                && let Some(user) = get_username()
            {
                footer.push_str(&format!(" \u{2502} user:{}", user));
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
            if inner_w > 130
                && let Ok(shell) = std::env::var("SHELL")
            {
                let shell_name = shell.rsplit('/').next().unwrap_or(&shell);
                footer.push_str(&format!(" \u{2502} sh:{}", shell_name));
            }
            if inner_w > 140
                && let Some(tty) = get_tty()
            {
                footer.push_str(&format!(" \u{2502} tty:{}", tty));
            }
            if inner_w > 150 {
                footer.push_str(&format!(" \u{2502} disks:{}", disk_count));
            }
            if inner_w > 160
                && let Some(bat) = get_battery()
            {
                footer.push_str(&format!(" \u{2502} bat:{}%", bat));
            }
            if inner_w > 190 {
                footer.push_str(&format!(" \u{2502} {}x{}", w, h));
            }

            if app.filter.active {
                footer.push_str(&format!(" \u{2502} FILTER> {}_", app.filter.buf));
            }
            let footer_display: String = footer.chars().take(inner_w as usize).collect();
            set_str(buf, lm, frow, &footer_display, footer_s, inner_w);
        }
    }

    // ─── Filter popup ───
    if app.filter.active {
        draw_filter_popup(buf, w, h, app);
    }

    // ─── Help overlay ───
    if app.show_help {
        draw_help(buf, w, h, app);
    }

    // ─── Theme chooser overlay ───
    if app.theme_chooser.active {
        draw_theme_chooser(buf, w, h, app);
    }

    // ─── Theme editor overlay ───
    if app.theme_edit.active {
        draw_theme_editor(buf, w, h, app);
    }

    // ─── Hover tooltip (1s delay for title/footer, right-click only for disk rows) ───
    if !app.show_help
        && !app.theme_edit.active
        && !app.theme_chooser.active
        && !app.filter.active
        && app.hover_ready()
    {
        match app.hovered_zone(h) {
            HoverZone::DiskRow(idx) => {
                if app.hover.right_click {
                    let disks = app.sorted_disks();
                    if let Some(disk) = disks.get(idx) {
                        draw_hover_tooltip(buf, w, h, app, disk);
                    }
                }
            }
            HoverZone::TitleBar => {
                if app.prefs.show_tooltips || app.hover.right_click {
                    draw_hover_bar_tooltip(buf, w, h, app, true);
                }
            }
            HoverZone::FooterBar => {
                if app.prefs.show_tooltips || app.hover.right_click {
                    draw_hover_bar_tooltip(buf, w, h, app, false);
                }
            }
            HoverZone::None => {}
        }
    }

    // ─── Status message (centered overlay above footer) ───
    if let Some((ref msg, t)) = app.status_msg
        && t.elapsed() < Duration::from_secs(3)
    {
        draw_status(buf, w, h, app, msg);
    }
}

fn draw_status(buf: &mut Buffer, w: u16, h: u16, app: &App, text: &str) {
    let (_, _, _, light_purple, _, _) = palette_for_prefs(&app.prefs);
    let msg_len = text.chars().count() as u16 + 4;
    let x0 = (w.saturating_sub(msg_len)) / 2;
    let bottom_offset: u16 = 6 + if app.prefs.show_header { 1 } else { 0 };
    let y0 = h.saturating_sub(bottom_offset);
    let s = Style::default().fg(Color::Black).bg(light_purple);
    set_str(buf, x0, y0, &format!("  {}  ", text), s, msg_len);
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
    let hint_s = Style::default().fg(Color::Indexed(240)).bg(HELP_BG);
    let label_s = Style::default().fg(Color::Indexed(141)).bg(HELP_BG);

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
    let current_val = if app.filter.text.is_empty() {
        "(none)"
    } else {
        &app.filter.text
    };
    set_str(buf, x0 + 2, y0 + 2, current_label, label_s, 8);
    set_str(
        buf,
        x0 + 10,
        y0 + 2,
        current_val,
        bg_s,
        box_w.saturating_sub(13),
    );

    let input_w = box_w.saturating_sub(4);
    let field_y = y0 + 3;
    for x in x0 + 2..x0 + 2 + input_w {
        set_cell(buf, x, field_y, " ", input_s);
    }
    set_str(buf, x0 + 2, field_y, "\u{25B8} ", input_s, 2);

    let max_visible = (input_w as usize).saturating_sub(3);
    let cursor_pos = app.filter.cursor;
    let buf_len = app.filter.buf.len();

    let (vis_start, vis_end) = if buf_len <= max_visible {
        (0, buf_len)
    } else if cursor_pos <= max_visible / 2 {
        (0, max_visible)
    } else if cursor_pos >= buf_len.saturating_sub(max_visible / 2) {
        (buf_len.saturating_sub(max_visible), buf_len)
    } else {
        (cursor_pos - max_visible / 2, cursor_pos + max_visible / 2)
    };

    let display_buf = &app.filter.buf[vis_start..vis_end.min(buf_len)];
    set_str(
        buf,
        x0 + 4,
        field_y,
        display_buf,
        input_s,
        input_w.saturating_sub(3),
    );

    let cursor_x = x0 + 4 + (cursor_pos - vis_start) as u16;
    if cursor_x < x0 + 2 + input_w {
        let ch = app.filter.buf.chars().nth(cursor_pos).unwrap_or(' ');
        let cursor_s = Style::default()
            .fg(Color::Indexed(235))
            .bg(Color::Indexed(48));
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
        for x in 1..w - 1 {
            set_cell(buf, x, 0, "\u{2550}", border_s);
        }
        set_cell(buf, w - 1, 0, "\u{2557}", border_s);
        set_cell(buf, 0, h - 1, "\u{255A}", border_s);
        for x in 1..w - 1 {
            set_cell(buf, x, h - 1, "\u{2550}", border_s);
        }
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
        let accent_s = Style::default()
            .fg(pal_blue)
            .bg(DARK_BG)
            .add_modifier(Modifier::BOLD);

        for x in lm..w.saturating_sub(rm) {
            set_cell(buf, x, row, " ", Style::default().bg(DARK_BG));
        }

        set_str(
            buf,
            lm,
            row,
            " \u{25B6} DRILL DOWN \u{25C0} ",
            accent_s,
            inner_w,
        );
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
        let hdr_s = Style::default()
            .fg(pal_lpurple)
            .add_modifier(Modifier::BOLD);
        let sort_arrow = if app.drill.sort_rev {
            "\u{25BC}"
        } else {
            "\u{25B2}"
        };
        let name_arrow = if app.drill.sort == DrillSortMode::Name {
            sort_arrow
        } else {
            " "
        };
        let size_arrow = if app.drill.sort == DrillSortMode::Size {
            sort_arrow
        } else {
            " "
        };
        let hdr = format!(
            "   {}{:<name_w$} {:>9}{}",
            name_arrow,
            "NAME",
            "SIZE",
            size_arrow,
            name_w = (inner_w as usize).saturating_sub(16)
        );
        set_str(buf, lm, row, &hdr, hdr_s, inner_w);
        row += 1;
        draw_separator(buf, row, w, show_border, border_s);
        row += 1;
    }

    // ─── Footer area ───
    let footer_rows: u16 = 2 + (if show_border { 1 } else { 0 });
    let entry_area_end = h.saturating_sub(footer_rows);

    // ─── Scanning indicator with progress bar ───
    if app.drill.scanning {
        let scan_count = *app.drill.scan_count.lock().unwrap();
        let scan_total = *app.drill.scan_total.lock().unwrap();
        let scanning_s = Style::default().fg(pal_blue).add_modifier(Modifier::BOLD);

        if scan_total > 0 {
            let label = format!(" \u{25CB} Scanning {}/{} ", scan_count, scan_total);
            let label_len = label.chars().count() as u16;
            set_str(buf, lm + 1, row, &label, scanning_s, inner_w);

            // Progress bar
            let bar_max = (inner_w as usize).saturating_sub(label_len as usize + 3);
            if bar_max > 4 {
                let frac = scan_count as f64 / scan_total as f64;
                let filled = (frac * bar_max as f64).round() as usize;
                let bar_x = lm + 1 + label_len;
                for j in 0..bar_max {
                    let x = bar_x + j as u16;
                    if j < filled {
                        let gc = gradient_color_at_prefs(j as f64 / bar_max as f64, &app.prefs);
                        set_cell(buf, x, row, "\u{2588}", Style::default().fg(gc));
                    } else {
                        set_cell(buf, x, row, "\u{2591}", Style::default().fg(DIM_BORDER));
                    }
                }
            }
        } else {
            set_str(
                buf,
                lm + 2,
                row,
                "\u{25CB} Scanning\u{2026}",
                scanning_s,
                inner_w,
            );
        }
        row += 1;
    }

    // ─── Entries ───
    let max_size = app
        .drill
        .entries
        .first()
        .map(|e| e.size)
        .unwrap_or(1)
        .max(1);

    for (i, entry) in app
        .drill
        .entries
        .iter()
        .enumerate()
        .skip(app.drill.scroll_offset)
    {
        if row >= entry_area_end {
            break;
        }

        let is_selected = i == app.drill.selected;

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
        let name_max =
            (inner_w as usize).saturating_sub(5 + size_col_w as usize + bar_col_w as usize + 2);
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
    if app.drill.entries.is_empty() && !app.drill.scanning {
        let empty_s = Style::default().fg(DIM_BORDER);
        set_str(
            buf,
            lm + 2,
            row,
            "(empty or access denied)",
            empty_s,
            inner_w,
        );
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

            let entry_count = app.drill.entries.len();
            let total_size: u64 = app.drill.entries.iter().map(|e| e.size).sum();
            let sort_name = match app.drill.sort {
                DrillSortMode::Size => "size",
                DrillSortMode::Name => "name",
            };
            let sort_dir = if app.drill.sort_rev {
                "\u{25BC}"
            } else {
                "\u{25B2}"
            };
            let footer = format!(
                " \u{27E6}drill\u{22B7}down\u{27E7} \u{25C0}\u{25C0}\u{25C0} items:{} \u{2502} total:{} \u{2502} sort:{}{} \u{2502} s:size \u{2502} n:name \u{2502} r:rev \u{2502} bksp:back",
                entry_count,
                format_bytes(total_size, app.prefs.unit_mode),
                sort_name,
                sort_dir,
            );
            let footer_display: String = footer.chars().take(inner_w as usize).collect();
            set_str(buf, lm, frow, &footer_display, footer_s, inner_w);
        }
    }

    // ─── Hover tooltip for drill-down entries (right-click only) ───
    if app.hover_ready()
        && app.hover.right_click
        && let Some(idx) = app.hovered_drill_index()
        && let Some(entry) = app.drill.entries.get(idx)
    {
        draw_hover_drill_tooltip(buf, w, h, app, entry);
    }
}

fn draw_hover_drill_tooltip(buf: &mut Buffer, w: u16, h: u16, app: &App, entry: &DirEntry) {
    let (hover_x, hover_y) = match app.hover.pos {
        Some(pos) => pos,
        None => return,
    };

    let mut lines: Vec<(String, String)> = Vec::new();
    let kind = if entry.is_dir {
        "\u{1F4C1} Directory"
    } else {
        "\u{25CB} File"
    };
    lines.push(("\u{25B6} Name".into(), entry.name.clone()));
    lines.push(("  Type".into(), kind.into()));
    lines.push((
        "  Size".into(),
        format_bytes(entry.size, app.prefs.unit_mode),
    ));
    // Show size in all unit modes for comparison
    if app.prefs.unit_mode != UnitMode::Bytes {
        lines.push(("  Size (bytes)".into(), format!("{} bytes", entry.size)));
    }
    if app.prefs.unit_mode != UnitMode::Human {
        lines.push((
            "  Size (human)".into(),
            format_bytes(entry.size, UnitMode::Human),
        ));
    }
    lines.push(("  Path".into(), entry.path.clone()));
    let parent_total: u64 = app.drill.entries.iter().map(|e| e.size).sum();
    if parent_total > 0 {
        let pct = (entry.size as f64 / parent_total as f64) * 100.0;
        lines.push(("  Share".into(), format!("{:.1}% of directory", pct)));
        // Visual share bar
        let filled = (pct / 5.0).round() as usize;
        let bar: String =
            "\u{2588}".repeat(filled) + &"\u{2591}".repeat(20usize.saturating_sub(filled));
        lines.push(("  Share bar".into(), bar));
    }
    // Rank within siblings
    let rank = app
        .drill
        .entries
        .iter()
        .filter(|e| e.size > entry.size)
        .count()
        + 1;
    let total_siblings = app.drill.entries.len();
    lines.push((
        "  Rank".into(),
        format!("#{} of {} entries", rank, total_siblings),
    ));
    // Current depth
    let depth = app.drill.path.len();
    lines.push((
        "  Depth".into(),
        format!("{} level{} deep", depth, if depth == 1 { "" } else { "s" }),
    ));
    lines.push((
        "  Parent".into(),
        app.drill.path.last().cloned().unwrap_or_default(),
    ));
    // Sort info
    lines.push((
        "  Sort".into(),
        format!(
            "{:?} {}",
            app.drill.sort,
            if app.drill.sort_rev {
                "\u{25BC} desc"
            } else {
                "\u{25B2} asc"
            }
        ),
    ));
    lines.push((
        "  Scan src".into(),
        "recursive read_dir() with progress".into(),
    ));
    lines.push((
        "  Actions".into(),
        "Enter=open  Bksp=up  s=sort  r=reverse".into(),
    ));

    render_tooltip(buf, w, h, hover_x, hover_y, app, &lines);
}

fn draw_hover_tooltip(buf: &mut Buffer, w: u16, h: u16, app: &App, disk: &DiskEntry) {
    let (hover_x, hover_y) = match app.hover.pos {
        Some(pos) => pos,
        None => return,
    };

    let mut lines: Vec<(String, String)> = vec![
        ("\u{25B6} Mount".into(), disk.mount.clone()),
        ("  Filesystem".into(), disk.fs.clone()),
        // Separator
        ("  \u{2500}\u{2500}\u{2500} Capacity".into(), "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}".into()),
        ("  Used".into(), format_bytes(disk.used, app.prefs.unit_mode)),
    ];
    let free = disk.total.saturating_sub(disk.used);
    lines.push(("  Free".into(), format_bytes(free, app.prefs.unit_mode)));
    lines.push((
        "  Total".into(),
        format_bytes(disk.total, app.prefs.unit_mode),
    ));
    // Show raw bytes alongside human-readable
    if app.prefs.unit_mode != UnitMode::Bytes {
        lines.push(("  Used (bytes)".into(), format!("{}", disk.used)));
        lines.push(("  Free (bytes)".into(), format!("{}", free)));
        lines.push(("  Total (bytes)".into(), format!("{}", disk.total)));
    }
    if app.prefs.unit_mode != UnitMode::Human {
        lines.push((
            "  Used (human)".into(),
            format_bytes(disk.used, UnitMode::Human),
        ));
        lines.push(("  Free (human)".into(), format_bytes(free, UnitMode::Human)));
        lines.push((
            "  Total (human)".into(),
            format_bytes(disk.total, UnitMode::Human),
        ));
    }
    lines.push(("  Usage".into(), format!("{:.1}%", disk.pct)));
    lines.push(("  Free %".into(), format!("{:.1}%", 100.0 - disk.pct)));
    // Visual usage bar
    let filled = (disk.pct / 5.0).round() as usize;
    let bar: String =
        "\u{2588}".repeat(filled) + &"\u{2591}".repeat(20usize.saturating_sub(filled));
    lines.push(("  Usage bar".into(), format!("{} {:.1}%", bar, disk.pct)));
    // Rank among all disks
    let sorted = app.sorted_disks();
    let rank_by_size = sorted.iter().filter(|d| d.total > disk.total).count() + 1;
    let rank_by_usage = sorted.iter().filter(|d| d.pct > disk.pct).count() + 1;
    let disk_count = sorted.len();
    lines.push((
        "  Rank (size)".into(),
        format!("#{} of {} volumes", rank_by_size, disk_count),
    ));
    lines.push((
        "  Rank (usage)".into(),
        format!("#{} of {} volumes", rank_by_usage, disk_count),
    ));
    lines.push(("  \u{2500}\u{2500}\u{2500} Hardware".into(), "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}".into()));
    lines.push((
        "  Kind".into(),
        match disk.kind {
            sysinfo::DiskKind::SSD => "SSD (Solid State Drive)".into(),
            sysinfo::DiskKind::HDD => "HDD (Hard Disk Drive)".into(),
            _ => "Unknown".into(),
        },
    ));
    let (thresh_status, thresh_desc) = if disk.pct >= app.prefs.thresh_crit as f64 {
        (
            "\u{2716} CRITICAL",
            format!("Above crit threshold ({}%)", app.prefs.thresh_crit),
        )
    } else if disk.pct >= app.prefs.thresh_warn as f64 {
        (
            "\u{26A0} WARNING",
            format!("Above warn threshold ({}%)", app.prefs.thresh_warn),
        )
    } else {
        (
            "\u{25C8} Nominal",
            format!("Below warn threshold ({}%)", app.prefs.thresh_warn),
        )
    };
    lines.push(("  Status".into(), thresh_status.into()));
    lines.push(("  Threshold".into(), thresh_desc));
    let headroom_crit = (app.prefs.thresh_crit as f64 - disk.pct).max(0.0);
    let headroom_warn = (app.prefs.thresh_warn as f64 - disk.pct).max(0.0);
    if disk.pct < app.prefs.thresh_crit as f64 {
        let bytes_to_crit = ((headroom_crit / 100.0) * disk.total as f64) as u64;
        lines.push((
            "  Headroom (crit)".into(),
            format!(
                "{:.1}% / {} until critical",
                headroom_crit,
                format_bytes(bytes_to_crit, UnitMode::Human)
            ),
        ));
    }
    if disk.pct < app.prefs.thresh_warn as f64 {
        let bytes_to_warn = ((headroom_warn / 100.0) * disk.total as f64) as u64;
        lines.push((
            "  Headroom (warn)".into(),
            format!(
                "{:.1}% / {} until warning",
                headroom_warn,
                format_bytes(bytes_to_warn, UnitMode::Human)
            ),
        ));
    }
    if let Some(smart) = disk.smart_status {
        lines.push(("  \u{2500}\u{2500}\u{2500} Health".into(), "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}".into()));
        let (smart_val, smart_src) = match smart {
            SmartHealth::Verified => (
                "\u{2714} Verified — drive healthy",
                "diskutil info (macOS) or /sys/block (Linux)",
            ),
            SmartHealth::Failing => (
                "\u{2718} FAILING — replace drive immediately!",
                "diskutil info (macOS) or /sys/block (Linux)",
            ),
            SmartHealth::Unknown => (
                "? Unknown — SMART not available",
                "SMART not supported or not available for this device",
            ),
        };
        lines.push(("  SMART".into(), smart_val.into()));
        lines.push(("  SMART src".into(), smart_src.into()));
    }
    if disk.latency_ms.is_some() || disk.io_read_rate.is_some() || disk.io_write_rate.is_some() {
        lines.push(("  \u{2500}\u{2500}\u{2500} I/O".into(), "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}".into()));
    }
    if let Some(lat) = disk.latency_ms {
        lines.push(("  Latency".into(), format_latency(lat)));
        let lat_rating = if lat < 1.0 {
            "Excellent"
        } else if lat < 10.0 {
            "Good"
        } else if lat < 100.0 {
            "Fair"
        } else if lat < 1000.0 {
            "Slow"
        } else {
            "Very slow"
        };
        lines.push(("  Lat rating".into(), lat_rating.into()));
        lines.push(("  Lat raw".into(), format!("{:.3} ms", lat)));
        lines.push((
            "  Lat src".into(),
            "timed read_dir() with 2s timeout".into(),
        ));
    }
    if let Some(rd) = disk.io_read_rate {
        lines.push(("  \u{25B2} Read".into(), format!("{}/s", format_rate(rd))));
        lines.push(("  Read raw".into(), format!("{:.0} bytes/s", rd)));
    }
    if let Some(wr) = disk.io_write_rate {
        lines.push(("  \u{25BC} Write".into(), format!("{}/s", format_rate(wr))));
        lines.push(("  Write raw".into(), format!("{:.0} bytes/s", wr)));
    }
    if disk.io_read_rate.is_some() || disk.io_write_rate.is_some() {
        let total_io = disk.io_read_rate.unwrap_or(0.0) + disk.io_write_rate.unwrap_or(0.0);
        lines.push((
            "  \u{25C6} Total I/O".into(),
            format!("{}/s", format_rate(total_io)),
        ));
        lines.push((
            "  I/O src".into(),
            "IOKit (macOS) or /proc/diskstats (Linux)".into(),
        ));
    }
    lines.push(("  \u{2500}\u{2500}\u{2500} Meta".into(), "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}".into()));
    if app.prefs.bookmarks.contains(&disk.mount) {
        lines.push((
            "  \u{2605} Pinned".into(),
            "Bookmarked — appears at top of list".into(),
        ));
    } else {
        lines.push((
            "  \u{2606} Not pinned".into(),
            "Press B to bookmark this volume".into(),
        ));
    }
    // Show what percentage this volume is of total system storage
    let (global_total, _) = dedup_disk_totals(&app.disks);
    if global_total > 0 {
        let vol_share = (disk.total as f64 / global_total as f64) * 100.0;
        lines.push((
            "  System share".into(),
            format!(
                "{:.1}% of total storage ({})",
                vol_share,
                format_bytes(global_total, UnitMode::Human)
            ),
        ));
    }
    lines.push((
        "  Source".into(),
        "getmntinfo (macOS) / /proc/mounts (Linux)".into(),
    ));
    lines.push((
        "  Data age".into(),
        format!("Refreshed every {}s", app.prefs.refresh_rate),
    ));
    lines.push((
        "  Actions".into(),
        "Enter=drill  o=open  y=copy  B=bookmark".into(),
    ));

    render_tooltip(buf, w, h, hover_x, hover_y, app, &lines);
}

/// Render a small tooltip popup with given lines near the hover cursor.
fn render_tooltip(
    buf: &mut Buffer,
    w: u16,
    h: u16,
    hover_x: u16,
    hover_y: u16,
    app: &App,
    lines: &[(String, String)],
) {
    let (_, pal_green, _, pal_lpurple, _, _) = palette_for_prefs(&app.prefs);
    let bc = border_color(app);
    let border_s = Style::default().fg(bc);
    let label_s = Style::default().fg(pal_lpurple).bg(HELP_BG);
    let val_s = Style::default().fg(pal_green).bg(HELP_BG);

    let max_label_w = lines
        .iter()
        .map(|(l, _)| l.chars().count())
        .max()
        .unwrap_or(6);
    let max_val_w = lines
        .iter()
        .map(|(_, v)| v.chars().count())
        .max()
        .unwrap_or(6);
    let box_w = (max_label_w + max_val_w + 5).min(w as usize - 4) as u16;
    let box_h = (lines.len() + 2) as u16;

    let x0 = if hover_x + box_w + 2 < w {
        hover_x + 2
    } else {
        w.saturating_sub(box_w + 2)
    };
    let y0 = if hover_y + box_h + 1 < h {
        hover_y + 1
    } else {
        h.saturating_sub(box_h + 1)
    };

    for y in y0..y0 + box_h {
        for x in x0..x0 + box_w {
            if x < w && y < h {
                set_cell(buf, x, y, " ", Style::default().bg(HELP_BG));
            }
        }
    }
    set_cell(buf, x0, y0, "\u{256D}", border_s);
    set_cell(buf, x0 + box_w - 1, y0, "\u{256E}", border_s);
    set_cell(buf, x0, y0 + box_h - 1, "\u{2570}", border_s);
    set_cell(buf, x0 + box_w - 1, y0 + box_h - 1, "\u{256F}", border_s);
    for x in x0 + 1..x0 + box_w - 1 {
        set_cell(buf, x, y0, "\u{2500}", border_s);
        set_cell(buf, x, y0 + box_h - 1, "\u{2500}", border_s);
    }
    for y in y0 + 1..y0 + box_h - 1 {
        set_cell(buf, x0, y, "\u{2502}", border_s);
        set_cell(buf, x0 + box_w - 1, y, "\u{2502}", border_s);
    }
    let lw = max_label_w + 1;
    for (i, (label, val)) in lines.iter().enumerate() {
        let y = y0 + 1 + i as u16;
        if y >= y0 + box_h - 1 {
            break;
        }
        set_str(buf, x0 + 1, y, label, label_s, lw as u16);
        if !val.is_empty() {
            let vx = x0 + 1 + lw as u16 + 1;
            let remaining = box_w.saturating_sub(lw as u16 + 3);
            set_str(buf, vx, y, val, val_s, remaining);
        }
    }
}

/// Find which pipe-delimited segment the cursor x falls into, return the segment text.
fn segment_at_x(rendered: &str, hover_x: u16, bar_start_x: u16) -> Option<String> {
    let rel_x = hover_x.saturating_sub(bar_start_x) as usize;
    let mut pos = 0usize;
    for segment in rendered.split('\u{2502}') {
        let seg_len = segment.chars().count() + 1; // +1 for the pipe
        if rel_x < pos + seg_len {
            return Some(segment.trim().to_string());
        }
        pos += seg_len;
    }
    // Last segment (no trailing pipe)
    rendered
        .split('\u{2502}')
        .next_back()
        .map(|s| s.trim().to_string())
}

/// Build tooltip lines for a title-bar segment.
fn title_segment_tooltip(segment: &str, app: &App) -> Vec<(String, String)> {
    let seg_lower = segment.to_lowercase();
    if seg_lower.contains("disk matrix") {
        vec![
            ("\u{25B6} App".into(), "STORAGESHOWER".into()),
            (
                "  Version".into(),
                format!("v{}", env!("CARGO_PKG_VERSION")),
            ),
            ("  Desc".into(), "Cyberpunk disk usage TUI monitor".into()),
            ("  Author".into(), "MenkeTechnologies".into()),
            (
                "  Repo".into(),
                "github.com/MenkeTechnologies/storageshower".into(),
            ),
            ("  Crate".into(), "crates.io/crates/storageshower".into()),
            ("  License".into(), "MIT".into()),
            ("  Install".into(), "cargo install storageshower".into()),
            (
                "  Platform".into(),
                format!("{} {}", app.stats.os_name, app.stats.arch),
            ),
            (
                "  Built with".into(),
                "Rust + ratatui + sysinfo + crossterm".into(),
            ),
            ("  Config".into(), "~/.storageshower.conf (TOML)".into()),
            ("  Volumes".into(), format!("{} mounted", app.disks.len())),
            ("  Uptime".into(), format_uptime(app.stats.uptime)),
        ]
    } else if seg_lower.starts_with("node:") {
        vec![
            ("\u{25B6} Hostname".into(), app.stats.hostname.clone()),
            (
                "  OS".into(),
                format!("{} {}", app.stats.os_name, app.stats.os_version),
            ),
            ("  Kernel".into(), app.stats.kernel.clone()),
            ("  Arch".into(), app.stats.arch.clone()),
            (
                "  CPU cores".into(),
                format!("{} logical", app.stats.cpu_count),
            ),
            (
                "  RAM".into(),
                format_bytes(app.stats.mem_total, UnitMode::Human),
            ),
            (
                "  Load".into(),
                format!(
                    "{:.2} / {:.2} / {:.2}",
                    app.stats.load_avg.0, app.stats.load_avg.1, app.stats.load_avg.2
                ),
            ),
            ("  Processes".into(), format!("{}", app.stats.process_count)),
            ("  Uptime".into(), format_uptime(app.stats.uptime)),
            ("  Source".into(), "sysinfo::System::host_name()".into()),
        ]
    } else if seg_lower.starts_with("date:") || seg_lower.starts_with("clock:") {
        let now = chrono_now();
        vec![
            ("\u{25B6} Date".into(), now.0),
            ("  Time".into(), now.1),
            ("  Uptime".into(), format_uptime(app.stats.uptime)),
            (
                "  Raw uptime".into(),
                format!("{} seconds", app.stats.uptime),
            ),
            ("  Timezone".into(), "System local timezone".into()),
            ("  Source".into(), "libc::localtime_r (UNIX epoch)".into()),
        ]
    } else if seg_lower.starts_with("load:") {
        let l = app.stats.load_avg;
        let cpus = app.stats.cpu_count as f64;
        let sat_1 = if cpus > 0.0 { l.0 / cpus * 100.0 } else { 0.0 };
        let sat_5 = if cpus > 0.0 { l.1 / cpus * 100.0 } else { 0.0 };
        let sat_15 = if cpus > 0.0 { l.2 / cpus * 100.0 } else { 0.0 };
        let trend = if l.0 > l.1 && l.1 > l.2 {
            "\u{25B2} Increasing"
        } else if l.0 < l.1 && l.1 < l.2 {
            "\u{25BC} Decreasing"
        } else {
            "\u{25C6} Stable"
        };
        vec![
            ("\u{25B6} Load Average".into(), String::new()),
            (
                "  1 min".into(),
                format!(
                    "{:.2} ({:.0}% of {} cores)",
                    l.0, sat_1, app.stats.cpu_count
                ),
            ),
            (
                "  5 min".into(),
                format!(
                    "{:.2} ({:.0}% of {} cores)",
                    l.1, sat_5, app.stats.cpu_count
                ),
            ),
            (
                "  15 min".into(),
                format!(
                    "{:.2} ({:.0}% of {} cores)",
                    l.2, sat_15, app.stats.cpu_count
                ),
            ),
            ("  Trend".into(), trend.into()),
            (
                "  Desc".into(),
                "Average runnable+uninterruptible processes".into(),
            ),
            ("  Ideal".into(), format!("< {:.1} (1.0 per core)", cpus)),
            ("  Source".into(), "sysinfo::System::load_average()".into()),
        ]
    } else if seg_lower.starts_with("mem:") {
        let pct = if app.stats.mem_total > 0 {
            (app.stats.mem_used as f64 / app.stats.mem_total as f64) * 100.0
        } else {
            0.0
        };
        let free = app.stats.mem_total.saturating_sub(app.stats.mem_used);
        let filled = (pct / 5.0).round() as usize;
        let bar: String =
            "\u{2588}".repeat(filled) + &"\u{2591}".repeat(20usize.saturating_sub(filled));
        let swap_pct = if app.stats.swap_total > 0 {
            (app.stats.swap_used as f64 / app.stats.swap_total as f64) * 100.0
        } else {
            0.0
        };
        vec![
            ("\u{25B6} Memory".into(), String::new()),
            (
                "  Used".into(),
                format_bytes(app.stats.mem_used, UnitMode::Human),
            ),
            ("  Free".into(), format_bytes(free, UnitMode::Human)),
            (
                "  Total".into(),
                format_bytes(app.stats.mem_total, UnitMode::Human),
            ),
            ("  Usage".into(), format!("{:.1}%", pct)),
            ("  Usage bar".into(), format!("{} {:.1}%", bar, pct)),
            ("  Used (bytes)".into(), format!("{}", app.stats.mem_used)),
            ("  Total (bytes)".into(), format!("{}", app.stats.mem_total)),
            (
                "  Per process".into(),
                format!(
                    "~{} across {} processes",
                    format_bytes(
                        if app.stats.process_count > 0 {
                            app.stats.mem_used / app.stats.process_count as u64
                        } else {
                            0
                        },
                        UnitMode::Human
                    ),
                    app.stats.process_count
                ),
            ),
            (
                "  Swap used".into(),
                format!(
                    "{} / {} ({:.1}%)",
                    format_bytes(app.stats.swap_used, UnitMode::Human),
                    format_bytes(app.stats.swap_total, UnitMode::Human),
                    swap_pct
                ),
            ),
            ("  Source".into(), "sysinfo::System::used_memory()".into()),
        ]
    } else if seg_lower.starts_with("cpu:") {
        let l = app.stats.load_avg;
        let cpus = app.stats.cpu_count;
        vec![
            ("\u{25B6} CPU".into(), String::new()),
            ("  Cores".into(), format!("{} logical", cpus)),
            ("  Arch".into(), app.stats.arch.clone()),
            ("  Load (1m)".into(), format!("{:.2}", l.0)),
            ("  Load (5m)".into(), format!("{:.2}", l.1)),
            ("  Load (15m)".into(), format!("{:.2}", l.2)),
            (
                "  Processes".into(),
                format!("{} running", app.stats.process_count),
            ),
            ("  Source".into(), "sysinfo::System::cpus().len()".into()),
        ]
    } else if seg_lower.starts_with("procs:") {
        let avg_mem = if app.stats.process_count > 0 {
            app.stats.mem_used / app.stats.process_count as u64
        } else {
            0
        };
        vec![
            (
                "\u{25B6} Processes".into(),
                format!("{}", app.stats.process_count),
            ),
            ("  Desc".into(), "Total running system processes".into()),
            (
                "  Avg memory".into(),
                format!("{} per process", format_bytes(avg_mem, UnitMode::Human)),
            ),
            (
                "  CPU cores".into(),
                format!("{} logical", app.stats.cpu_count),
            ),
            ("  Load (1m)".into(), format!("{:.2}", app.stats.load_avg.0)),
            (
                "  Source".into(),
                "sysinfo::System::processes().len()".into(),
            ),
        ]
    } else if seg_lower.starts_with("swap:") {
        let swap_free = app.stats.swap_total.saturating_sub(app.stats.swap_used);
        let swap_pct = if app.stats.swap_total > 0 {
            (app.stats.swap_used as f64 / app.stats.swap_total as f64) * 100.0
        } else {
            0.0
        };
        let filled = (swap_pct / 5.0).round() as usize;
        let bar: String =
            "\u{2588}".repeat(filled) + &"\u{2591}".repeat(20usize.saturating_sub(filled));
        let mem_pct = if app.stats.mem_total > 0 {
            (app.stats.mem_used as f64 / app.stats.mem_total as f64) * 100.0
        } else {
            0.0
        };
        vec![
            ("\u{25B6} Swap".into(), String::new()),
            (
                "  Used".into(),
                format_bytes(app.stats.swap_used, UnitMode::Human),
            ),
            ("  Free".into(), format_bytes(swap_free, UnitMode::Human)),
            (
                "  Total".into(),
                format_bytes(app.stats.swap_total, UnitMode::Human),
            ),
            ("  Usage".into(), format!("{:.1}%", swap_pct)),
            ("  Usage bar".into(), format!("{} {:.1}%", bar, swap_pct)),
            ("  Used (bytes)".into(), format!("{}", app.stats.swap_used)),
            (
                "  Total (bytes)".into(),
                format!("{}", app.stats.swap_total),
            ),
            (
                "  RAM usage".into(),
                format!(
                    "{:.1}% ({} / {})",
                    mem_pct,
                    format_bytes(app.stats.mem_used, UnitMode::Human),
                    format_bytes(app.stats.mem_total, UnitMode::Human)
                ),
            ),
            (
                "  Desc".into(),
                "Virtual memory on disk when RAM is full".into(),
            ),
            ("  Source".into(), "sysinfo::System::used_swap()".into()),
        ]
    } else if seg_lower.starts_with("kern:") {
        vec![
            ("\u{25B6} Kernel".into(), app.stats.kernel.clone()),
            (
                "  OS".into(),
                format!("{} {}", app.stats.os_name, app.stats.os_version),
            ),
            ("  Arch".into(), app.stats.arch.clone()),
            ("  Uptime".into(), format_uptime(app.stats.uptime)),
            ("  Hostname".into(), app.stats.hostname.clone()),
            (
                "  Source".into(),
                "sysinfo::System::kernel_version()".into(),
            ),
        ]
    } else if seg_lower.starts_with("arch:") {
        vec![
            ("\u{25B6} Architecture".into(), app.stats.arch.clone()),
            ("  Kernel".into(), app.stats.kernel.clone()),
            (
                "  OS".into(),
                format!("{} {}", app.stats.os_name, app.stats.os_version),
            ),
            (
                "  CPU cores".into(),
                format!("{} logical", app.stats.cpu_count),
            ),
            ("  Source".into(), "sysinfo::System::cpu_arch()".into()),
        ]
    } else if seg_lower.contains("paused") {
        vec![
            ("\u{25B6} \u{23F8} PAUSED".into(), String::new()),
            ("  Desc".into(), "Data refresh is paused".into()),
            ("  Effect".into(), "Disk and system stats are frozen".into()),
            (
                "  Refresh rate".into(),
                format!("{}s (when active)", app.prefs.refresh_rate),
            ),
            ("  Volumes".into(), format!("{} mounted", app.disks.len())),
            ("  Resume".into(), "Press p to resume live data".into()),
        ]
    } else if seg_lower.contains("h=help") {
        vec![
            ("\u{25B6} Help".into(), String::new()),
            ("  Open".into(), "Press h / H / ? to show keybinds".into()),
            ("  Close".into(), "Same keys or Esc to dismiss".into()),
            ("  Layout".into(), "3-column keybind reference".into()),
            (
                "  Categories".into(),
                "Navigation, Display, Sort, I/O, Theme".into(),
            ),
            (
                "  Mouse".into(),
                "Click headers, scroll, R-click for tips".into(),
            ),
        ]
    } else {
        vec![("\u{25B6} Info".into(), segment.to_string())]
    }
}

/// Build tooltip lines for a footer-bar segment.
fn footer_segment_tooltip(segment: &str, app: &App) -> Vec<(String, String)> {
    let seg_lower = segment.to_lowercase();
    if seg_lower.contains("cyberdeck") || seg_lower.contains("zpwr") {
        let (total_storage, total_used) = dedup_disk_totals(&app.disks);
        let total_free = total_storage.saturating_sub(total_used);
        let overall_pct = if total_storage > 0 {
            (total_used as f64 / total_storage as f64) * 100.0
        } else {
            0.0
        };
        vec![
            ("\u{25B6} App".into(), "STORAGESHOWER".into()),
            (
                "  Version".into(),
                format!("v{}", env!("CARGO_PKG_VERSION")),
            ),
            ("  Desc".into(), "Cyberpunk disk usage TUI monitor".into()),
            ("  Author".into(), "MenkeTechnologies".into()),
            (
                "  Repo".into(),
                "github.com/MenkeTechnologies/storageshower".into(),
            ),
            ("  Crate".into(), "crates.io/crates/storageshower".into()),
            (
                "  Config".into(),
                "~/.storageshower.conf (auto-saved)".into(),
            ),
            ("  License".into(), "MIT".into()),
            (
                "  Storage".into(),
                format!(
                    "{} used / {} total ({:.1}%)",
                    format_bytes(total_used, UnitMode::Human),
                    format_bytes(total_storage, UnitMode::Human),
                    overall_pct
                ),
            ),
            ("  Free".into(), format_bytes(total_free, UnitMode::Human)),
            ("  Volumes".into(), format!("{} mounted", app.disks.len())),
            (
                "  Refresh".into(),
                format!("Every {}s", app.prefs.refresh_rate),
            ),
            ("  Uptime".into(), format_uptime(app.stats.uptime)),
        ]
    } else if seg_lower.starts_with("vol:") {
        let total = app.disks.len();
        let visible: usize = segment.trim_start_matches("vol:").parse().unwrap_or(0);
        let hidden = total.saturating_sub(visible);
        let (total_storage, total_used) = dedup_disk_totals(&app.disks);
        let overall_pct = if total_storage > 0 {
            (total_used as f64 / total_storage as f64) * 100.0
        } else {
            0.0
        };
        let ssd_count = app
            .disks
            .iter()
            .filter(|d| matches!(d.kind, sysinfo::DiskKind::SSD))
            .count();
        let hdd_count = app
            .disks
            .iter()
            .filter(|d| matches!(d.kind, sysinfo::DiskKind::HDD))
            .count();
        let other_count = total.saturating_sub(ssd_count + hdd_count);
        let fs_types: Vec<String> = {
            let mut fss: Vec<String> = app.disks.iter().map(|d| d.fs.clone()).collect();
            fss.sort();
            fss.dedup();
            fss
        };
        vec![
            ("\u{25B6} Volumes".into(), format!("{} visible", visible)),
            ("  Total mounted".into(), format!("{} filesystems", total)),
            ("  Hidden".into(), format!("{} (filtered out)", hidden)),
            ("  SSDs".into(), format!("{}", ssd_count)),
            ("  HDDs".into(), format!("{}", hdd_count)),
            ("  Other".into(), format!("{}", other_count)),
            ("  FS types".into(), fs_types.join(", ")),
            (
                "  Total storage".into(),
                format_bytes(total_storage, UnitMode::Human),
            ),
            (
                "  Total used".into(),
                format!(
                    "{} ({:.1}%)",
                    format_bytes(total_used, UnitMode::Human),
                    overall_pct
                ),
            ),
            (
                "  Total free".into(),
                format_bytes(total_storage.saturating_sub(total_used), UnitMode::Human),
            ),
            (
                "  Filters".into(),
                "show_all / show_local / filter text".into(),
            ),
            ("  Keys".into(), "a=show all  l=local only  /=filter".into()),
        ]
    } else if seg_lower.starts_with("sort:") {
        let sorted = app.sorted_disks();
        let first = sorted.first().map(|d| d.mount.as_str()).unwrap_or("(none)");
        let last = sorted.last().map(|d| d.mount.as_str()).unwrap_or("(none)");
        vec![
            (
                "\u{25B6} Sort Mode".into(),
                format!("{:?}", app.prefs.sort_mode),
            ),
            (
                "  Direction".into(),
                if app.prefs.sort_rev {
                    "Descending \u{25BC}"
                } else {
                    "Ascending \u{25B2}"
                }
                .into(),
            ),
            ("  First".into(), first.into()),
            ("  Last".into(), last.into()),
            ("  Volumes".into(), format!("{} sorted", sorted.len())),
            ("  Keys".into(), "n=name  u=usage%  s=size".into()),
            ("  Reverse".into(), "r or press same sort key again".into()),
            (
                "  Mouse".into(),
                "Click column header to sort/reverse".into(),
            ),
            ("  Config".into(), "sort_mode / sort_rev in prefs".into()),
        ]
    } else if seg_lower.ends_with('s')
        && seg_lower.chars().next().is_some_and(|c| c.is_ascii_digit())
    {
        let rates = [1u64, 2, 5, 10];
        let current = app.prefs.refresh_rate;
        let idx = rates.iter().position(|&r| r == current).unwrap_or(0);
        let next = rates[(idx + 1) % rates.len()];
        let prev = rates[(idx + rates.len() - 1) % rates.len()];
        vec![
            ("\u{25B6} Refresh Rate".into(), format!("{}s", current)),
            (
                "  Desc".into(),
                "How often disk data is re-collected".into(),
            ),
            ("  Next (f)".into(), format!("{}s", next)),
            ("  Prev (F)".into(), format!("{}s", prev)),
            (
                "  Cycle".into(),
                "1s \u{2192} 2s \u{2192} 5s \u{2192} 10s".into(),
            ),
            (
                "  Paused".into(),
                if app.paused {
                    "Yes (press p)"
                } else {
                    "No — live updates"
                }
                .into(),
            ),
            (
                "  Source".into(),
                "Background thread via Arc<Mutex<>>".into(),
            ),
            ("  Config".into(), "refresh_rate in prefs".into()),
        ]
    } else if seg_lower == "gradient"
        || seg_lower == "solid"
        || seg_lower == "thin"
        || seg_lower == "ascii"
    {
        let styles = ["Gradient", "Solid", "Thin", "Ascii"];
        let current_idx = match app.prefs.bar_style {
            BarStyle::Gradient => 0,
            BarStyle::Solid => 1,
            BarStyle::Thin => 2,
            BarStyle::Ascii => 3,
        };
        let next_idx = (current_idx + 1) % styles.len();
        vec![
            (
                "\u{25B6} Bar Style".into(),
                format!("{:?}", app.prefs.bar_style),
            ),
            (
                "  Desc".into(),
                "Visual style of usage progress bars".into(),
            ),
            (
                "  Gradient".into(),
                "\u{2588}\u{2593}\u{2592}\u{2591} color-graded fill".into(),
            ),
            (
                "  Solid".into(),
                "\u{2588}\u{2588}\u{2588}\u{2588} single-color fill".into(),
            ),
            (
                "  Thin".into(),
                "\u{2501}\u{2501}\u{2501}\u{2501} horizontal line".into(),
            ),
            ("  Ascii".into(), "#### portable ASCII chars".into()),
            ("  Next (b)".into(), styles[next_idx].into()),
            ("  Config".into(), "bar_style in prefs".into()),
            (
                "  CLI".into(),
                "--bar-style <gradient|solid|thin|ascii>".into(),
            ),
        ]
    } else if seg_lower.starts_with("up:") {
        let secs = app.stats.uptime;
        let days = secs / 86400;
        let hours = (secs % 86400) / 3600;
        let mins = (secs % 3600) / 60;
        vec![
            ("\u{25B6} System Uptime".into(), format_uptime(secs)),
            ("  Days".into(), format!("{}", days)),
            ("  Hours".into(), format!("{}", hours)),
            ("  Minutes".into(), format!("{}", mins)),
            ("  Raw".into(), format!("{} seconds", secs)),
            ("  Hostname".into(), app.stats.hostname.clone()),
            ("  Kernel".into(), app.stats.kernel.clone()),
            ("  Source".into(), "sysinfo::System::uptime()".into()),
        ]
    } else if seg_lower.starts_with("user:") {
        let username = segment.trim_start_matches("user:");
        vec![
            ("\u{25B6} User".into(), username.into()),
            ("  Hostname".into(), app.stats.hostname.clone()),
            (
                "  OS".into(),
                format!("{} {}", app.stats.os_name, app.stats.os_version),
            ),
            ("  Arch".into(), app.stats.arch.clone()),
            (
                "  Shell".into(),
                std::env::var("SHELL").unwrap_or_else(|_| "unknown".into()),
            ),
            ("  Source".into(), "$USER or $USERNAME env var".into()),
        ]
    } else if seg_lower.starts_with("ip:") {
        let ip = segment.trim_start_matches("ip:");
        vec![
            ("\u{25B6} Local IP".into(), ip.into()),
            ("  Desc".into(), "Primary network interface address".into()),
            ("  Method".into(), "UDP socket bind to 8.8.8.8:80".into()),
            (
                "  Note".into(),
                "No data is sent — only local addr probed".into(),
            ),
            ("  Hostname".into(), app.stats.hostname.clone()),
            ("  Source".into(), "std::net::UdpSocket::connect()".into()),
        ]
    } else if seg_lower.starts_with("os:") {
        vec![
            (
                "\u{25B6} OS".into(),
                format!("{} {}", app.stats.os_name, app.stats.os_version),
            ),
            ("  Kernel".into(), app.stats.kernel.clone()),
            ("  Arch".into(), app.stats.arch.clone()),
            ("  Hostname".into(), app.stats.hostname.clone()),
            (
                "  CPU cores".into(),
                format!("{} logical", app.stats.cpu_count),
            ),
            (
                "  RAM".into(),
                format_bytes(app.stats.mem_total, UnitMode::Human),
            ),
            ("  Uptime".into(), format_uptime(app.stats.uptime)),
            (
                "  Source".into(),
                "sysinfo::System::name() + os_version()".into(),
            ),
        ]
    } else if seg_lower.starts_with("sh:") {
        let shell = segment.trim_start_matches("sh:");
        vec![
            ("\u{25B6} Shell".into(), shell.into()),
            ("  Desc".into(), "Login shell for current user".into()),
            ("  Env var".into(), "$SHELL".into()),
            (
                "  User".into(),
                std::env::var("USER").unwrap_or_else(|_| "unknown".into()),
            ),
            ("  Source".into(), "$SHELL environment variable".into()),
        ]
    } else if seg_lower.starts_with("tty:") {
        let tty = segment.trim_start_matches("tty:");
        vec![
            ("\u{25B6} TTY".into(), tty.into()),
            ("  Desc".into(), "Terminal device for this session".into()),
            ("  FD".into(), "stdin (fd 0)".into()),
            ("  Source".into(), "libc::ttyname(0)".into()),
        ]
    } else if seg_lower.starts_with("bat:") {
        let bat_str = segment.trim_start_matches("bat:");
        let bat_val: u8 = bat_str.trim_end_matches('%').parse().unwrap_or(0);
        let status = if bat_val > 80 {
            "\u{2714} Good"
        } else if bat_val > 20 {
            "\u{25C8} OK"
        } else {
            "\u{26A0} Low"
        };
        let filled = (bat_val as usize) / 5;
        let bar: String =
            "\u{2588}".repeat(filled) + &"\u{2591}".repeat(20usize.saturating_sub(filled));
        vec![
            ("\u{25B6} Battery".into(), bat_str.into()),
            ("  Status".into(), status.into()),
            ("  Level bar".into(), format!("{} {}%", bar, bat_val)),
            (
                "  Source".into(),
                "pmset (macOS) or /sys/class/power_supply (Linux)".into(),
            ),
        ]
    } else if seg_lower.starts_with("disks:") {
        let count_str = segment.trim_start_matches("disks:");
        let (total_storage, total_used) = dedup_disk_totals(&app.disks);
        let overall_pct = if total_storage > 0 {
            (total_used as f64 / total_storage as f64) * 100.0
        } else {
            0.0
        };
        vec![
            ("\u{25B6} Disk Count".into(), count_str.into()),
            ("  Desc".into(), "Total visible filesystems".into()),
            ("  Total mounted".into(), format!("{}", app.disks.len())),
            (
                "  Total storage".into(),
                format_bytes(total_storage, UnitMode::Human),
            ),
            ("  Overall usage".into(), format!("{:.1}%", overall_pct)),
            (
                "  Source".into(),
                "getmntinfo (macOS) / /proc/mounts (Linux)".into(),
            ),
        ]
    } else if seg_lower.contains("filter>") {
        let filter_len = app.filter.buf.len();
        let matched = app.sorted_disks().len();
        let total = app.disks.len();
        vec![
            ("\u{25B6} Filter Active".into(), app.filter.buf.clone()),
            ("  Length".into(), format!("{} chars", filter_len)),
            (
                "  Matched".into(),
                format!("{} of {} volumes", matched, total),
            ),
            (
                "  Hidden".into(),
                format!("{} filtered out", total.saturating_sub(matched)),
            ),
            (
                "  Type".into(),
                "Case-insensitive mount path substring".into(),
            ),
            (
                "  Cursor".into(),
                format!("Position {} of {}", app.filter.cursor, filter_len),
            ),
            (
                "  Nav keys".into(),
                "Enter=confirm  Esc=cancel  /=open".into(),
            ),
            (
                "  Edit keys".into(),
                "Ctrl+a/e=home/end  Ctrl+b/f=left/right".into(),
            ),
            (
                "  Delete".into(),
                "Ctrl+w=word  Ctrl+u=line  Ctrl+k=to-end".into(),
            ),
            (
                "  Clear".into(),
                "0 to clear filter (when not in filter mode)".into(),
            ),
        ]
    } else {
        let color_name = if let Some(ref n) = app.prefs.active_theme {
            n.clone()
        } else {
            app.prefs.color_mode.name().into()
        };
        if segment.trim() == color_name {
            let (pal_b, pal_g, pal_p, pal_lp, pal_r, pal_dp) = palette_for_prefs(&app.prefs);
            fn idx(c: Color) -> String {
                match c {
                    Color::Indexed(n) => format!("{}", n),
                    _ => "?".into(),
                }
            }
            return vec![
                ("\u{25B6} Color Theme".into(), color_name),
                (
                    "  Builtins".into(),
                    format!("{} palettes", ColorMode::ALL.len()),
                ),
                (
                    "  Custom".into(),
                    format!("{} user themes", app.prefs.custom_themes.len()),
                ),
                (
                    "  Total".into(),
                    format!(
                        "{} available themes",
                        ColorMode::ALL.len() + app.prefs.custom_themes.len()
                    ),
                ),
                (
                    "  Palette".into(),
                    format!(
                        "blue={} green={} purple={}",
                        idx(pal_b),
                        idx(pal_g),
                        idx(pal_p)
                    ),
                ),
                (
                    "  Palette+".into(),
                    format!(
                        "lpurple={} royal={} dark={}",
                        idx(pal_lp),
                        idx(pal_r),
                        idx(pal_dp)
                    ),
                ),
                (
                    "  Preview".into(),
                    "c=chooser (live preview + mouse click)".into(),
                ),
                ("  Editor".into(), "C=create custom themes".into()),
                ("  CLI".into(), "--color, --theme, --export-theme".into()),
                (
                    "  Config".into(),
                    "color_mode / active_theme in prefs".into(),
                ),
            ];
        }
        let unit_name = match app.prefs.unit_mode {
            UnitMode::Human => "human",
            UnitMode::GiB => "GiB",
            UnitMode::MiB => "MiB",
            UnitMode::Bytes => "bytes",
        };
        if segment.trim() == unit_name {
            let (total_storage, _) = dedup_disk_totals(&app.disks);
            return vec![
                ("\u{25B6} Unit Mode".into(), unit_name.into()),
                ("  Desc".into(), "How byte sizes are formatted".into()),
                (
                    "  Human".into(),
                    format!(
                        "Auto-scale ({} example)",
                        format_bytes(total_storage, UnitMode::Human)
                    ),
                ),
                (
                    "  GiB".into(),
                    format!(
                        "Gibibytes ({} example)",
                        format_bytes(total_storage, UnitMode::GiB)
                    ),
                ),
                (
                    "  MiB".into(),
                    format!(
                        "Mebibytes ({} example)",
                        format_bytes(total_storage, UnitMode::MiB)
                    ),
                ),
                (
                    "  Bytes".into(),
                    format!(
                        "Raw bytes ({} example)",
                        format_bytes(total_storage, UnitMode::Bytes)
                    ),
                ),
                (
                    "  Key".into(),
                    "i to cycle forward, I to cycle backward".into(),
                ),
                ("  CLI".into(), "--unit <human|gib|mib|bytes>".into()),
                ("  Config".into(), "unit_mode in prefs".into()),
            ];
        }
        vec![("Info".into(), segment.to_string())]
    }
}

fn draw_hover_bar_tooltip(buf: &mut Buffer, w: u16, h: u16, app: &App, is_title: bool) {
    let (hover_x, hover_y) = match app.hover.pos {
        Some(pos) => pos,
        None => return,
    };

    let lm: u16 = if app.prefs.show_border { 1 } else { 0 };

    // Read the rendered bar text from the buffer at the hover row
    let mut bar_text = String::new();
    for x in lm..w.saturating_sub(lm) {
        let cell = &buf[(x, hover_y)];
        bar_text.push_str(cell.symbol());
    }

    let segment = match segment_at_x(&bar_text, hover_x, lm) {
        Some(s) if !s.is_empty() => s,
        _ => return,
    };

    let lines = if is_title {
        title_segment_tooltip(&segment, app)
    } else {
        footer_segment_tooltip(&segment, app)
    };

    render_tooltip(buf, w, h, hover_x, hover_y, app, &lines);
}

fn draw_theme_chooser(buf: &mut Buffer, w: u16, h: u16, app: &App) {
    let themes = app.all_themes();
    let box_w: u16 = 50u16.min(w.saturating_sub(4));
    let box_h: u16 = (themes.len() as u16 + 4).min(h.saturating_sub(4));
    let x0 = (w.saturating_sub(box_w)) / 2;
    let y0 = (h.saturating_sub(box_h)) / 2;
    let bc = border_color(app);
    let border_s = Style::default().fg(bc);
    let title_s = Style::default()
        .fg(Color::Indexed(27))
        .bg(HELP_BG)
        .add_modifier(Modifier::BOLD);
    let hint_s = Style::default().fg(Color::Indexed(240)).bg(HELP_BG);
    let sel_s = Style::default().fg(Color::White).bg(Color::Indexed(237));
    let name_s = Style::default().fg(Color::Indexed(48)).bg(HELP_BG);

    // Background
    for y in y0..y0 + box_h {
        for x in x0..x0 + box_w {
            if x < w && y < h {
                set_cell(buf, x, y, " ", Style::default().bg(HELP_BG));
            }
        }
    }

    // Border
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
    let title = "\u{1F3A8} CHOOSE THEME";
    let tlen = title.chars().count() as u16;
    let tx = x0 + (box_w.saturating_sub(tlen)) / 2;
    set_str(buf, tx, y0 + 1, title, title_s, box_w - 2);

    // Theme list
    let content_start = y0 + 2;
    let content_end = y0 + box_h - 2;
    let visible = (content_end - content_start) as usize;

    // Scroll the list if needed
    let scroll = if app.theme_chooser.selected >= visible {
        app.theme_chooser.selected - visible + 1
    } else {
        0
    };

    // Determine current active theme key
    let current_key = if let Some(ref name) = app.prefs.active_theme {
        name.clone()
    } else {
        format!("{:?}", app.prefs.color_mode).to_lowercase()
    };

    for (i, (key, display)) in themes.iter().enumerate().skip(scroll) {
        let row_y = content_start + (i - scroll) as u16;
        if row_y >= content_end {
            break;
        }

        let is_sel = i == app.theme_chooser.selected;
        let is_active = *key == current_key;

        // Selection highlight
        if is_sel {
            for x in x0 + 1..x0 + box_w - 1 {
                set_cell(buf, x, row_y, " ", sel_s);
            }
        }

        // Active marker
        let marker = if is_active { "\u{25B8}" } else { " " };
        let row_style = if is_sel { sel_s } else { name_s };
        set_str(buf, x0 + 2, row_y, marker, row_style, 1);

        // Theme name
        set_str(buf, x0 + 4, row_y, display, row_style, 20);

        // Color swatch — get palette for this theme
        let swatch_x = x0 + 25;
        let colors: [u8; 6] = if let Some(theme) = app.prefs.custom_themes.get(key) {
            [
                theme.blue,
                theme.green,
                theme.purple,
                theme.light_purple,
                theme.royal,
                theme.dark_purple,
            ]
        } else {
            // Builtin — find the mode
            let mode = ColorMode::ALL
                .iter()
                .find(|&&m| format!("{:?}", m).to_lowercase() == *key)
                .copied()
                .unwrap_or(ColorMode::Default);
            let (a, b, c, d, e, f) = palette(mode);
            fn idx(col: Color) -> u8 {
                match col {
                    Color::Indexed(n) => n,
                    _ => 0,
                }
            }
            [idx(a), idx(b), idx(c), idx(d), idx(e), idx(f)]
        };
        for (j, &ci) in colors.iter().enumerate() {
            let sx = swatch_x + (j as u16 * 3);
            if sx + 2 < x0 + box_w - 1 {
                let swatch_style = Style::default().fg(Color::Indexed(ci));
                set_str(buf, sx, row_y, "\u{2588}\u{2588}", swatch_style, 2);
            }
        }
    }

    // Footer hint
    let hint_y = y0 + box_h - 1;
    let hint = " j/k/click:nav  Enter:select  Esc:cancel ";
    let hx = x0 + (box_w.saturating_sub(hint.len() as u16)) / 2;
    set_str(buf, hx, hint_y, hint, hint_s, box_w - 2);
}

fn draw_theme_editor(buf: &mut Buffer, w: u16, h: u16, app: &App) {
    let box_w: u16 = 56u16.min(w.saturating_sub(4));
    let box_h: u16 = if app.theme_edit.naming { 16 } else { 15 };
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
    let labels = [
        "blue",
        "green",
        "purple",
        "light_purple",
        "royal",
        "dark_purple",
    ];
    let colors = app.theme_edit.colors;

    for (i, label) in labels.iter().enumerate() {
        let row_y = y0 + 3 + i as u16;
        let is_sel = i == app.theme_edit.slot;

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
        set_str(
            buf,
            x0 + 24,
            row_y,
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            swatch_style,
            5,
        );

        // Gradient preview bar using all 6 current colors as a mini bar
        let preview_color = Color::Indexed(colors[i]);
        set_str(
            buf,
            x0 + 30,
            row_y,
            " \u{25C0}\u{2500}\u{2500}\u{25B6}",
            Style::default().fg(preview_color).bg(HELP_BG),
            5,
        );
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
        set_cell(
            buf,
            x0 + 11 + j as u16,
            preview_y,
            "\u{2588}",
            Style::default().fg(c).bg(HELP_BG),
        );
    }

    // Naming prompt or keybind hints
    if app.theme_edit.naming {
        let name_y = y0 + 12;
        let input_s = Style::default()
            .fg(Color::Indexed(48))
            .bg(Color::Indexed(235));
        set_str(buf, x0 + 2, name_y, "Theme name:", bg_s, 11);
        let name_display = format!("{}_", app.theme_edit.name);
        set_str(buf, x0 + 14, name_y, &name_display, input_s, box_w - 16);
        set_str(
            buf,
            x0 + 2,
            name_y + 1,
            "Enter:save  Esc:back",
            hint_s,
            box_w - 4,
        );
    } else {
        let hint_y = y0 + 12;
        set_str(
            buf,
            x0 + 2,
            hint_y,
            "j/k:select  h/l:\u{00B1}1  H/L:\u{00B1}10",
            hint_s,
            box_w - 4,
        );
        set_str(
            buf,
            x0 + 2,
            hint_y + 1,
            "Enter/s:save  Esc/q:cancel",
            hint_s,
            box_w - 4,
        );
    }
}

fn draw_help(buf: &mut Buffer, w: u16, h: u16, app: &App) {
    let box_w: u16 = 120u16.min(w.saturating_sub(4));
    let box_h: u16 = 48u16.min(h.saturating_sub(4));
    let x0 = (w.saturating_sub(box_w)) / 2;
    let y0 = (h.saturating_sub(box_h)) / 2;
    let (pal_blue, pal_green, _, pal_lpurple, _, pal_dpurple) = palette_for_prefs(&app.prefs);
    let bc = border_color(app);
    let border_s = Style::default().fg(bc);
    let bg_s = Style::default().fg(Color::White).bg(HELP_BG);
    let key_s = Style::default().fg(pal_green).bg(HELP_BG);
    let val_s = Style::default().fg(pal_lpurple).bg(HELP_BG);
    let title_s = Style::default()
        .fg(pal_blue)
        .bg(HELP_BG)
        .add_modifier(Modifier::BOLD);
    let section_s = Style::default()
        .fg(pal_dpurple)
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

    let title = format!(
        "\u{2328} DISK MATRIX v{} \u{2014} KEYBOARD SHORTCUTS",
        env!("CARGO_PKG_VERSION")
    );
    let tlen = title.chars().count() as u16;
    let tx = x0 + (box_w.saturating_sub(tlen)) / 2;
    set_str(buf, tx, y0 + 1, &title, title_s, box_w - 2);

    let byline = "by MenkeTechnologies";
    let byline_s = Style::default().fg(Color::Indexed(240)).bg(HELP_BG);
    let bx = x0 + (box_w.saturating_sub(byline.len() as u16)) / 2;
    set_str(buf, bx, y0 + 2, byline, byline_s, box_w - 2);

    struct HelpEntry {
        key: &'static str,
        desc: &'static str,
        val_fn: fn(&App) -> String,
        is_section: bool,
    }

    fn empty_val(_: &App) -> String {
        String::new()
    }

    // Organize entries into 3 columns for better visibility
    let col1 = vec![
        HelpEntry {
            key: "GENERAL",
            desc: "",
            val_fn: empty_val,
            is_section: true,
        },
        HelpEntry {
            key: "q/Q",
            desc: "Quit",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "h/H/?",
            desc: "Toggle help",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "p/P",
            desc: "Pause/resume",
            val_fn: |a| format!("[{}]", if a.paused { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "f/F",
            desc: "Cycle refresh",
            val_fn: |a| format!("[{}s]", a.prefs.refresh_rate),
            is_section: false,
        },
        HelpEntry {
            key: "l/L",
            desc: "Local only",
            val_fn: |a| format!("[{}]", if a.prefs.show_local { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "a/A",
            desc: "All filesystems",
            val_fn: |a| format!("[{}]", if a.prefs.show_all { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "SORT",
            desc: "",
            val_fn: empty_val,
            is_section: true,
        },
        HelpEntry {
            key: "n/N",
            desc: "By name",
            val_fn: |a| {
                if a.prefs.sort_mode == SortMode::Name {
                    "[\u{2713}]".into()
                } else {
                    String::new()
                }
            },
            is_section: false,
        },
        HelpEntry {
            key: "u/U",
            desc: "By usage %",
            val_fn: |a| {
                if a.prefs.sort_mode == SortMode::Pct {
                    "[\u{2713}]".into()
                } else {
                    String::new()
                }
            },
            is_section: false,
        },
        HelpEntry {
            key: "s/S",
            desc: "By size",
            val_fn: |a| {
                if a.prefs.sort_mode == SortMode::Size {
                    "[\u{2713}]".into()
                } else {
                    String::new()
                }
            },
            is_section: false,
        },
        HelpEntry {
            key: "r/R",
            desc: "Reverse",
            val_fn: |a| {
                format!(
                    "[{}]",
                    if a.prefs.sort_rev {
                        "\u{25BC}"
                    } else {
                        "\u{25B2}"
                    }
                )
            },
            is_section: false,
        },
        HelpEntry {
            key: "FILTER",
            desc: "",
            val_fn: empty_val,
            is_section: true,
        },
        HelpEntry {
            key: "/",
            desc: "Filter mode",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "0",
            desc: "Clear filter",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "NAV",
            desc: "",
            val_fn: empty_val,
            is_section: true,
        },
        HelpEntry {
            key: "j/k",
            desc: "Select next/prev",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "G/End",
            desc: "Jump to last",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "^G/Home",
            desc: "Jump to first",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "^D/^U",
            desc: "Half-page dn/up",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "Esc",
            desc: "Deselect",
            val_fn: empty_val,
            is_section: false,
        },
    ];

    let col2 = vec![
        HelpEntry {
            key: "DISPLAY",
            desc: "",
            val_fn: empty_val,
            is_section: true,
        },
        HelpEntry {
            key: "b",
            desc: "Bar style",
            val_fn: |a| {
                format!(
                    "[{}]",
                    match a.prefs.bar_style {
                        BarStyle::Gradient => "grad",
                        BarStyle::Solid => "solid",
                        BarStyle::Thin => "thin",
                        BarStyle::Ascii => "ascii",
                    }
                )
            },
            is_section: false,
        },
        HelpEntry {
            key: "c",
            desc: "Theme chooser",
            val_fn: |a| {
                let n = if let Some(ref t) = a.prefs.active_theme {
                    t.clone()
                } else {
                    a.prefs.color_mode.name().into()
                };
                format!("[{}]", n)
            },
            is_section: false,
        },
        HelpEntry {
            key: "C",
            desc: "Theme editor",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "v/V",
            desc: "Toggle bars",
            val_fn: |a| format!("[{}]", if a.prefs.show_bars { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "d/D",
            desc: "Used/size cols",
            val_fn: |a| format!("[{}]", if a.prefs.show_used { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "g",
            desc: "Col headers",
            val_fn: |a| format!("[{}]", if a.prefs.show_header { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "x/X",
            desc: "Border",
            val_fn: |a| format!("[{}]", if a.prefs.show_border { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "m/M",
            desc: "Compact mounts",
            val_fn: |a| format!("[{}]", if a.prefs.compact { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "w/W",
            desc: "Full paths",
            val_fn: |a| format!("[{}]", if a.prefs.full_mount { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "i/I",
            desc: "Cycle units",
            val_fn: |a| {
                format!(
                    "[{}]",
                    match a.prefs.unit_mode {
                        UnitMode::Human => "human",
                        UnitMode::GiB => "GiB",
                        UnitMode::MiB => "MiB",
                        UnitMode::Bytes => "B",
                    }
                )
            },
            is_section: false,
        },
        HelpEntry {
            key: "t",
            desc: "Warn threshold",
            val_fn: |a| format!("[{}%]", a.prefs.thresh_warn),
            is_section: false,
        },
        HelpEntry {
            key: "T",
            desc: "Hover tips",
            val_fn: |a| format!("[{}]", if a.prefs.show_tooltips { "on" } else { "off" }),
            is_section: false,
        },
        HelpEntry {
            key: "z/Z",
            desc: "Crit threshold",
            val_fn: |a| format!("[{}%]", a.prefs.thresh_crit),
            is_section: false,
        },
    ];

    let col3 = vec![
        HelpEntry {
            key: "ACTIONS",
            desc: "",
            val_fn: empty_val,
            is_section: true,
        },
        HelpEntry {
            key: "Enter",
            desc: "Drill down",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "o/O",
            desc: "Open in finder",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "y/Y",
            desc: "Copy path",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "e/E",
            desc: "Export to file",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "B",
            desc: "Bookmark \u{2605}",
            val_fn: |a| format!("[{}]", a.prefs.bookmarks.len()),
            is_section: false,
        },
        HelpEntry {
            key: "DRILL DOWN",
            desc: "",
            val_fn: empty_val,
            is_section: true,
        },
        HelpEntry {
            key: "Enter",
            desc: "Into subdir",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "Bksp",
            desc: "Up one level",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "Esc",
            desc: "Back to disks",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "s/n",
            desc: "Sort size/name",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "r",
            desc: "Reverse sort",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "o/O",
            desc: "Open directory",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "MOUSE",
            desc: "",
            val_fn: empty_val,
            is_section: true,
        },
        HelpEntry {
            key: "Click",
            desc: "Select row",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "Click\u{00D7}2",
            desc: "Drill into",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "Drag",
            desc: "Resize cols",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "R-Click",
            desc: "Mount tooltip",
            val_fn: empty_val,
            is_section: false,
        },
        HelpEntry {
            key: "Hover",
            desc: "Bar tooltip",
            val_fn: empty_val,
            is_section: false,
        },
    ];

    let columns = [col1, col2, col3];
    let col_w = ((box_w as usize).saturating_sub(4)) / 3;

    for (ci, col_entries) in columns.iter().enumerate() {
        let cx = x0 + 2 + (ci as u16) * col_w as u16;
        for (ri, entry) in col_entries.iter().enumerate() {
            let ey = y0 + 4 + ri as u16;
            if ey >= y0 + box_h - 1 {
                break;
            }
            if entry.is_section {
                set_str(buf, cx, ey, entry.key, section_s, col_w as u16);
            } else {
                set_str(buf, cx, ey, entry.key, key_s, 8);
                set_str(buf, cx + 9, ey, entry.desc, bg_s, 15);
                let val = (entry.val_fn)(app);
                if !val.is_empty() {
                    let vx = cx + 25;
                    let vw = col_w.saturating_sub(26);
                    set_str(buf, vx, ey, &val, val_s, vw as u16);
                }
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
        use crate::app::App;
        use std::sync::{Arc, Mutex};

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
        use crate::app::App;
        use std::sync::{Arc, Mutex};

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
        use crate::app::App;
        use std::sync::{Arc, Mutex};

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs.thresh_warn = 70;

        let (_, bg, icon) = thresh_color(70.0, &app);
        assert!(bg.is_some());
        assert_eq!(icon, "\u{26A0}");
    }

    #[test]
    fn thresh_color_at_exact_crit() {
        use crate::app::App;
        use std::sync::{Arc, Mutex};

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs.thresh_crit = 90;

        let (_, bg, icon) = thresh_color(90.0, &app);
        assert!(bg.is_some());
        assert_eq!(icon, "\u{2716}");
    }

    #[test]
    fn thresh_color_just_below_warn() {
        use crate::app::App;
        use std::sync::{Arc, Mutex};

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);
        app.prefs.thresh_warn = 70;

        let (_, bg, icon) = thresh_color(69.9, &app);
        assert!(bg.is_none());
        assert_eq!(icon, "\u{25C8}");
    }

    #[test]
    fn thresh_color_at_zero() {
        use crate::app::App;
        use std::sync::{Arc, Mutex};

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let app = App::new_default(shared);

        let (_, bg, icon) = thresh_color(0.0, &app);
        assert!(bg.is_none());
        assert_eq!(icon, "\u{25C8}");
    }

    #[test]
    fn thresh_color_at_hundred() {
        use crate::app::App;
        use std::sync::{Arc, Mutex};

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let app = App::new_default(shared);

        let (_, bg, icon) = thresh_color(100.0, &app);
        assert!(bg.is_some());
        assert_eq!(icon, "\u{2716}");
    }

    // ── border_color with different color modes ─────────────

    #[test]
    fn border_color_different_modes() {
        use crate::app::App;
        use std::sync::{Arc, Mutex};

        let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
        let mut app = App::new_default(shared);

        for &mode in ColorMode::ALL {
            app.prefs.color_mode = mode;
            app.paused = false;
            let c = border_color(&app);
            assert_ne!(
                c,
                Color::Reset,
                "border_color with {:?} returned Reset",
                mode
            );
        }
    }

    #[test]
    fn palette_for_prefs_uses_custom_theme_indices() {
        use crate::prefs::Prefs;
        use crate::types::ThemeColors;

        let mut prefs = Prefs::default();
        prefs.active_theme = Some("cyber".into());
        prefs.custom_themes.insert(
            "cyber".into(),
            ThemeColors {
                blue: 33,
                green: 44,
                purple: 55,
                light_purple: 66,
                royal: 77,
                dark_purple: 88,
            },
        );
        let (a, b, c, d, e, f) = palette_for_prefs(&prefs);
        assert_eq!(a, Color::Indexed(33));
        assert_eq!(b, Color::Indexed(44));
        assert_eq!(c, Color::Indexed(55));
        assert_eq!(d, Color::Indexed(66));
        assert_eq!(e, Color::Indexed(77));
        assert_eq!(f, Color::Indexed(88));
    }

    #[test]
    fn gradient_color_at_out_of_range_frac_still_resolves() {
        let neg = gradient_color_at(-1.0, ColorMode::Green);
        let over = gradient_color_at(10.0, ColorMode::Purple);
        assert_ne!(neg, Color::Reset);
        assert_ne!(over, Color::Reset);
    }

    // ── segment_at_x (title/footer bar hit testing) ─────────

    #[test]
    fn segment_at_x_first_column() {
        let bar = "Alpha\u{2502}Beta\u{2502}Gamma";
        assert_eq!(super::segment_at_x(bar, 0, 0), Some("Alpha".into()));
        assert_eq!(super::segment_at_x(bar, 2, 0), Some("Alpha".into()));
    }

    #[test]
    fn segment_at_x_second_column() {
        let bar = "Alpha\u{2502}Beta\u{2502}Gamma";
        // First column is rel_x 0..5 (segment len 5 + pipe = 6); rel_x 6 is second column.
        assert_eq!(super::segment_at_x(bar, 6, 0), Some("Beta".into()));
    }

    #[test]
    fn segment_at_x_respects_bar_start_offset() {
        let bar = "X\u{2502}Y";
        assert_eq!(super::segment_at_x(bar, 10, 10), Some("X".into()));
        assert_eq!(super::segment_at_x(bar, 12, 10), Some("Y".into()));
    }

    #[test]
    fn segment_at_x_no_pipes_whole_string() {
        let bar = "only";
        assert_eq!(super::segment_at_x(bar, 0, 0), Some("only".into()));
    }
}
