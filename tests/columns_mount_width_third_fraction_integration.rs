//! `mount_col_width` uses one-third of inner width when not compact (default prefs).

use storageshower::columns::mount_col_width;
use storageshower::prefs::Prefs;

#[test]
fn inner_90_yields_thirty() {
    let p = Prefs::default();
    assert_eq!(mount_col_width(90, &p), 30);
}

#[test]
fn inner_150_yields_fifty() {
    let p = Prefs::default();
    assert_eq!(mount_col_width(150, &p), 50);
}
