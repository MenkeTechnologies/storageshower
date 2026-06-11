//! `mount_col_width` must not panic on a narrow terminal when a custom mount
//! width is configured.
//!
//! Bug class: `clamp(min, max)` panics when `min > max`. The implementation
//! computes `col_mount_w.clamp(8, inner_w.saturating_sub(20))`. For any
//! `inner_w < 28` the upper bound drops below the lower bound (8), so the
//! `clamp` precondition `min <= max` is violated and the process aborts.
//! A disk-usage TUI rendering in a sub-28-column pane (split tmux panes,
//! narrow side panels) with `col_mount_w > 0` set in prefs hits this on
//! every frame.

use storageshower::prefs::Prefs;

#[test]
fn mount_col_width_narrow_inner_with_custom_does_not_panic() {
    let mut p = Prefs::default();
    p.col_mount_w = 25; // custom width configured

    // inner_w = 27 -> saturating_sub(20) = 7 -> clamp(8, 7) -> min > max panic.
    let w = storageshower::columns::mount_col_width(27, &p);

    // Width is consumed as a column count; it must stay within the pane.
    assert!(
        w <= 27,
        "mount width {w} exceeds the {}-column inner area",
        27
    );
}

#[test]
fn mount_col_width_one_column_inner_with_custom_does_not_panic() {
    let mut p = Prefs::default();
    p.col_mount_w = 30;

    // inner_w = 1 -> saturating_sub(20) = 0 -> clamp(8, 0) -> min > max panic.
    let w = storageshower::columns::mount_col_width(1, &p);
    assert!(w <= 1, "mount width {w} exceeds the 1-column inner area");
}
