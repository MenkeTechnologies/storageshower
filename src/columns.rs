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
    let pct_w = if app.prefs.col_pct_w > 0 {
        app.prefs.col_pct_w as usize
    } else {
        5
    };
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
    use crate::app::App;
    use crate::testutil::test_app;
    use crate::types::{SysStats, UnitMode};
    use std::sync::{Arc, Mutex};

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

    #[test]
    fn right_col_width_dynamic_at_least_min_with_sample_disks() {
        let mut app = test_app();
        app.prefs.col_bar_end_w = 0;
        app.prefs.show_used = true;
        let w = right_col_width(&app);
        assert!(w >= 22);
    }

    #[test]
    fn right_col_width_wider_with_bytes_unit_mode() {
        let mut app = test_app();
        app.prefs.unit_mode = UnitMode::Bytes;
        app.prefs.col_bar_end_w = 0;
        let w_bytes = right_col_width(&app);
        app.prefs.unit_mode = UnitMode::Human;
        let w_human = right_col_width(&app);
        assert!(w_bytes >= w_human);
    }

    #[test]
    fn right_col_width_custom_pct_column_wide() {
        let mut app = test_app();
        app.prefs.col_pct_w = 12;
        app.prefs.col_bar_end_w = 0;
        let w = right_col_width(&app);
        assert!(w >= 22);
    }

    #[test]
    fn mount_col_width_at_tiny_terminal_uses_floor() {
        let p = Prefs::default();
        let w = mount_col_width(24, &p);
        assert_eq!(w, 12);
    }

    #[test]
    fn mount_col_width_narrow_inner_uses_twelve_floor() {
        let p = Prefs::default();
        let w = mount_col_width(20, &p);
        assert_eq!(w, 12);
    }

    #[test]
    fn mount_col_width_third_of_inner_when_above_floor() {
        let p = Prefs::default();
        assert_eq!(mount_col_width(90, &p), 30);
    }

    #[test]
    fn right_col_width_static_no_used_custom_bar_end_still_clamps_min() {
        let mut p = Prefs::default();
        p.show_used = false;
        p.col_bar_end_w = 3;
        assert_eq!(right_col_width_static(&p), 5);
    }

    #[test]
    fn mount_col_width_quarter_terminal_default() {
        let p = Prefs::default();
        assert_eq!(mount_col_width(100, &p), 33);
    }
}
