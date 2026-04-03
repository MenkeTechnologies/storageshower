//! `storageshower::columns` width helpers from the public crate API.

use storageshower::columns::{mount_col_width, right_col_width_static};
use storageshower::prefs::Prefs;

#[test]
fn right_col_static_with_explicit_bar_end_respects_min() {
    let p = Prefs {
        col_bar_end_w: 1,
        ..Default::default()
    };
    assert_eq!(right_col_width_static(&p), 5);
}

#[test]
fn mount_col_custom_clamped_to_inner_minus_twenty() {
    let p = Prefs {
        col_mount_w: 99,
        ..Default::default()
    };
    assert_eq!(mount_col_width(50, &p), 30);
}

#[test]
fn mount_col_auto_uses_third_noncompact() {
    let p = Prefs::default();
    assert_eq!(mount_col_width(99, &p), 33);
}
