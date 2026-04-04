//! `scan_directory` with five files (size-descending order).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn five_files_largest_first() {
    let dir = tempdir().expect("tempdir");
    for (i, len) in [(0usize, 2), (1, 8), (2, 64), (3, 256), (4, 4096)] {
        fs::write(dir.path().join(format!("f{i}")), vec![0u8; len]).expect("w");
    }
    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 5);
    assert_eq!(v[0].name, "f4");
    assert!(v[0].size >= v[4].size);
}
