//! `scan_directory` with three files (sorted by size descending).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn three_files_largest_first() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("tiny"), [0u8; 1]).expect("tiny");
    fs::write(dir.path().join("mid"), [0u8; 100]).expect("mid");
    fs::write(dir.path().join("huge"), [0u8; 500]).expect("huge");

    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 3);
    assert_eq!(v[0].name, "huge");
    assert_eq!(v[2].name, "tiny");
}
