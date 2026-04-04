//! `scan_directory` on paths that do not exist or are unreadable.

use storageshower::system::scan_directory;

#[test]
fn nonexistent_path_returns_empty() {
    let v = scan_directory("/no/such/path/storageshower_test_xyz");
    assert!(v.is_empty());
}

#[test]
fn empty_string_returns_empty() {
    let v = scan_directory("");
    assert!(v.is_empty());
}
