//! `scan_directory` returns entries sorted by size descending (largest first).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn sorts_files_by_size_largest_first() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("tiny"), b"x").expect("write tiny");
    fs::write(dir.path().join("huge"), vec![0u8; 8000]).expect("write huge");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "huge");
    assert_eq!(entries[1].name, "tiny");
    assert!(entries[0].size >= entries[1].size);
}

#[test]
fn empty_directory_returns_empty() {
    let dir = tempdir().expect("tempdir");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    assert!(entries.is_empty());
}

#[test]
fn single_file_one_entry() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("only.bin"), [1u8, 2, 3]).expect("write");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "only.bin");
    assert_eq!(entries[0].size, 3);
    assert!(!entries[0].is_dir);
}
