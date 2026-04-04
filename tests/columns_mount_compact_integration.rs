//! `mount_col_width` when `prefs.compact` is true (fixed 16).

use storageshower::columns::mount_col_width;
use storageshower::prefs::Prefs;

#[test]
fn compact_ignores_wide_terminal() {
    let p = Prefs {
        compact: true,
        ..Default::default()
    };
    assert_eq!(mount_col_width(500, &p), 16);
}

#[test]
fn explicit_col_mount_w_takes_precedence_over_compact() {
    let p = Prefs {
        compact: true,
        col_mount_w: 99,
        ..Default::default()
    };
    assert_eq!(mount_col_width(120, &p), 99);
}
