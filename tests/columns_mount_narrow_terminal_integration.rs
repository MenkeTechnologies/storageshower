//! `mount_col_width` with a narrow inner width (floor 12 for default prefs).

use storageshower::columns::mount_col_width;
use storageshower::prefs::Prefs;

#[test]
fn narrow_inner_uses_twelve_floor() {
    let p = Prefs::default();
    assert_eq!(mount_col_width(20, &p), 12);
}

#[test]
fn eighteen_wide_uses_twelve_not_six() {
    let p = Prefs::default();
    assert_eq!(mount_col_width(18, &p), 12);
}
