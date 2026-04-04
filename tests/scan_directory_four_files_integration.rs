//! `scan_directory` with four files (descending size order).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn four_files_sorted_largest_first() {
    let dir = tempdir().expect("tempdir");
    for (name, len) in [("a", 1usize), ("b", 40), ("c", 200), ("d", 1000)] {
        fs::write(dir.path().join(name), vec![0u8; len]).expect(name);
    }
    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 4);
    assert_eq!(v[0].name, "d");
    assert_eq!(v[3].name, "a");
}
