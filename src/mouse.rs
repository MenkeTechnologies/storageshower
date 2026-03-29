use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use std::time::Instant;

use crate::app::App;
use crate::columns::{mount_col_width, right_col_width};
use crate::types::*;

impl App {
    pub fn handle_mouse(&mut self, event: MouseEvent, term_w: u16, term_h: u16) {
        // Cancel hover timer whenever mouse moves to a new position.
        // Specific handlers below re-enable it only for valid hover zones.
        let hover_moved = if matches!(event.kind, MouseEventKind::Moved) {
            let new_pos = (event.column, event.row);
            if self.hover.pos != Some(new_pos) {
                self.hover.pos = Some(new_pos);
                self.hover.since = None;
                self.hover.right_click = false;
                true
            } else {
                false
            }
        } else {
            false
        };

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
                        self.theme_chooser.selected =
                            (self.theme_chooser.selected + 1).min(count - 1);
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
        let pct_w: u16 = if self.prefs.col_pct_w > 0 {
            self.prefs.col_pct_w
        } else {
            5
        };
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
                // Pos/flags already updated at top of handle_mouse.
                // Re-enable timer only when position landed in a valid hover zone.
                if hover_moved {
                    if self.drill.mode == ViewMode::DrillDown {
                        if self.hovered_drill_index().is_some() {
                            self.hover.since = Some(Instant::now());
                        }
                    } else if self.hovered_zone(term_h) != HoverZone::None {
                        self.hover.since = Some(Instant::now());
                    }
                }
            }
            MouseEventKind::ScrollDown => {
                if self.drill.mode == ViewMode::DrillDown {
                    if !self.drill.entries.is_empty() {
                        self.drill.selected =
                            (self.drill.selected + 1).min(self.drill.entries.len() - 1);
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

#[cfg(test)]
mod tests {

    use crate::columns::{mount_col_width, right_col_width};

    use crate::testutil::*;
    use crate::types::*;
    use crossterm::event::{KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

    // ── Mouse handling ────────────────────────────────────

    #[test]
    fn mouse_right_click_triggers_hover() {
        let mut app = test_app();
        assert!(app.hover.pos.is_none());
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Right),
                column: 15,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
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
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: mount_sep_x,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(matches!(app.drag, Some(DragTarget::MountSep)));

        // Drag to new position
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Drag(MouseButton::Left),
                column: 30,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.prefs.col_mount_w > 0);

        // Release
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Up(MouseButton::Left),
                column: 30,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.drag.is_none());
    }

    #[test]
    fn mouse_up_without_drag_noop() {
        let mut app = test_app();
        assert!(app.drag.is_none());
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Up(MouseButton::Left),
                column: 10,
                row: 10,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        // No crash, drag is still None
        assert!(app.drag.is_none());
    }

    #[test]
    fn mouse_scroll_and_other_events_noop() {
        let mut app = test_app();
        let prev_help = app.show_help;
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::ScrollUp,
                column: 10,
                row: 10,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
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
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 10,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert_eq!(app.selected, Some(0));
    }

    #[test]
    fn mouse_click_selects_second_disk() {
        let mut app = test_app();
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 10,
                row: 6,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert_eq!(app.selected, Some(1));
    }

    #[test]
    fn mouse_click_out_of_range_no_select() {
        let mut app = test_app();
        // Click far below disk rows (row 50 is way past the 4 disks)
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 10,
                row: 50,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.selected.is_none());
    }

    #[test]
    fn mouse_click_already_selected_enters_drilldown() {
        let mut app = test_app();
        // First click selects
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 10,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert_eq!(app.selected, Some(0));
        assert_eq!(app.drill.mode, ViewMode::Disks);

        // Second click on same row enters drill-down
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 10,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert_eq!(app.drill.mode, ViewMode::DrillDown);
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
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 30,
                row: content_y + 1,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
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
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 0,
                row: 0,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
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
            MouseEvent {
                kind: MouseEventKind::ScrollDown,
                column: 40,
                row: 12,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert_eq!(app.theme_chooser.selected, 1);
        // Auto-applied
        assert_eq!(app.prefs.color_mode, ColorMode::ALL[1]);
        // Scroll up
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::ScrollUp,
                column: 40,
                row: 12,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
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
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Right),
                column: 15,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
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
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Right),
                column: 15,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.hover.right_click);
        // Move mouse
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 20,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(!app.hover.right_click);
        assert_eq!(app.hover.pos, Some((20, 5)));
    }

    #[test]
    fn mouse_move_same_pos_keeps_right_click_flag() {
        let mut app = test_app();
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Right),
                column: 15,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.hover.right_click);
        // Move to same position — should not clear
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 15,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.hover.right_click);
    }

    // ── Hover timer early cancellation on move ──────────────

    #[test]
    fn mouse_move_cancels_hover_before_theme_chooser() {
        let mut app = test_app();
        // Move to title bar to start hover timer
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 10,
                row: 1,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.hover.since.is_some());

        // Open theme chooser
        app.handle_key(make_key(KeyCode::Char('c')));
        assert!(app.theme_chooser.active);

        // Move mouse while theme chooser is open — hover timer must be cancelled
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 20,
                row: 10,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.hover.since.is_none());
        assert_eq!(app.hover.pos, Some((20, 10)));
    }

    #[test]
    fn mouse_move_in_theme_chooser_same_pos_preserves_hover() {
        let mut app = test_app();
        // Set up hover at a position, then open theme chooser
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 10,
                row: 1,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        let since_before = app.hover.since;
        app.handle_key(make_key(KeyCode::Char('c')));

        // Move to SAME position — should not reset
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 10,
                row: 1,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert_eq!(app.hover.since, since_before);
    }

    // ── Hover auto-hide (non-right-click) ─────────────────

    #[test]
    fn hover_ready_false_before_delay() {
        let mut app = test_app();
        app.hover.since = Some(std::time::Instant::now());
        app.hover.right_click = false;
        assert!(!app.hover_ready());
    }

    #[test]
    fn hover_auto_hides_after_4s() {
        let mut app = test_app();
        // Simulate hover started 5 seconds ago (past the 4s auto-hide window)
        app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_secs(5));
        app.hover.right_click = false;
        assert!(!app.hover_ready());
    }

    #[test]
    fn hover_visible_within_window() {
        let mut app = test_app();
        // Simulate hover started 2 seconds ago (past 1s delay, within 4s auto-hide)
        app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_millis(2000));
        app.hover.right_click = false;
        assert!(app.hover_ready());
    }

    #[test]
    fn right_click_hover_does_not_auto_hide() {
        let mut app = test_app();
        // Simulate right-click hover started 10 seconds ago
        app.hover.since = Some(std::time::Instant::now() - std::time::Duration::from_secs(10));
        app.hover.right_click = true;
        assert!(app.hover_ready());
    }

    // ── Hover timer cancellation outside detection zone ──────

    #[test]
    fn mouse_move_to_empty_zone_cancels_hover_timer() {
        let mut app = test_app();
        // Move to title bar (hover zone)
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 10,
                row: 1,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.hover.since.is_some());
        // Move to border row 0 (HoverZone::None)
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 10,
                row: 0,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.hover.since.is_none());
    }

    #[test]
    fn mouse_move_within_zone_keeps_hover_timer() {
        let mut app = test_app();
        // Move to a disk row (valid zone)
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 10,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.hover.since.is_some());
        // Move to another disk row (still valid zone)
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Moved,
                column: 10,
                row: 6,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.hover.since.is_some());
    }

    // ── Drag priority over header sort ────────────────────────

    #[test]
    fn drag_pct_sep_takes_priority_over_sort() {
        let mut app = test_app();
        app.prefs.show_used = true;
        let right_w = right_col_width(&app);
        let pct_w: u16 = if app.prefs.col_pct_w > 0 {
            app.prefs.col_pct_w
        } else {
            5
        };
        let right_start = 80u16.saturating_sub(1 + right_w);
        let pct_sep_x = right_start + pct_w;
        let header_row: u16 = 3; // show_border default = true, so header_row = 3

        // Click at pct separator on header row
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: pct_sep_x,
                row: header_row,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
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
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: bar_end_x,
                row: header_row,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(matches!(app.drag, Some(DragTarget::BarEndSep)));
    }

    #[test]
    fn drag_pct_sep_and_release() {
        let mut app = test_app();
        app.prefs.show_used = true;
        let right_w = right_col_width(&app);
        let pct_w: u16 = if app.prefs.col_pct_w > 0 {
            app.prefs.col_pct_w
        } else {
            5
        };
        let right_start = 80u16.saturating_sub(1 + right_w);
        let pct_sep_x = right_start + pct_w;

        // Start drag
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: pct_sep_x,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(matches!(app.drag, Some(DragTarget::PctSep)));

        // Drag to new position
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Drag(MouseButton::Left),
                column: pct_sep_x + 3,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.prefs.col_pct_w > 0);

        // Release
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Up(MouseButton::Left),
                column: pct_sep_x + 3,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
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
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: bar_end_x,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(matches!(app.drag, Some(DragTarget::BarEndSep)));

        // Drag
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Drag(MouseButton::Left),
                column: bar_end_x - 5,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.prefs.col_bar_end_w > 0);

        // Release
        app.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Up(MouseButton::Left),
                column: bar_end_x - 5,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            80,
            24,
        );
        assert!(app.drag.is_none());
    }
}
