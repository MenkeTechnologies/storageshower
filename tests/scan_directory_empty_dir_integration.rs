//! `scan_directory` on an empty temporary directory.

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn empty_directory_yields_empty_vec() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().to_str().expect("utf8 path");
    let v = scan_directory(path);
    assert!(v.is_empty(), "expected no entries, got {}", v.len());
}
