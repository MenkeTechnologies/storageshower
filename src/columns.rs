use crate::app::App;
use crate::helpers::format_bytes;
use crate::prefs::Prefs;

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
    for d in disks {
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
