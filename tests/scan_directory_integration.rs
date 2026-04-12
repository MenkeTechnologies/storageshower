//! Filesystem-backed tests for `scan_directory` / `scan_directory_with_progress`.
//! Uses tempfile only; safe for headless Linux CI.

use std::sync::{Arc, Mutex};
use storageshower::system::{scan_directory, scan_directory_with_progress};

#[test]
fn scan_directory_nonexistent_path_returns_empty() {
    let p = "/this/path/should/not/exist/storageshower_scan_xyz";
    assert!(scan_directory(p).is_empty());
}

#[test]
fn scan_directory_empty_directory() {
    let dir = tempfile::tempdir().expect("tempdir");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    assert!(entries.is_empty());
}

#[test]
fn scan_directory_sorts_by_size_descending() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::write(root.join("small.bin"), vec![0u8; 10]).expect("write");
    std::fs::write(root.join("big.bin"), vec![0u8; 500]).expect("write");
    std::fs::write(root.join("mid.bin"), vec![0u8; 100]).expect("write");

    let entries = scan_directory(root.to_str().expect("utf8"));
    assert_eq!(entries.len(), 3);
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, vec!["big.bin", "mid.bin", "small.bin"]);
    assert!(entries[0].size >= entries[1].size && entries[1].size >= entries[2].size);
    assert!(!entries.iter().any(|e| e.is_dir));
}

#[test]
fn scan_directory_directory_entry_includes_nested_bytes() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    let sub = root.join("nested");
    std::fs::create_dir_all(&sub).expect("mkdir");
    std::fs::write(sub.join("a"), vec![1u8; 40]).expect("write");
    std::fs::write(sub.join("b"), vec![2u8; 60]).expect("write");

    let entries = scan_directory(root.to_str().expect("utf8"));
    let nested = entries
        .iter()
        .find(|e| e.name == "nested")
        .expect("nested row");
    assert!(nested.is_dir);
    assert_eq!(nested.size, 100, "dir_size should sum nested files");
}

#[test]
fn scan_directory_with_progress_sets_total_and_final_count() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    for i in 0..5u8 {
        std::fs::write(root.join(format!("f{i}.txt")), [i; 3]).expect("write");
    }

    let count = Arc::new(Mutex::new(0usize));
    let total = Arc::new(Mutex::new(0usize));
    let c = Arc::clone(&count);
    let t = Arc::clone(&total);

    let entries = scan_directory_with_progress(root.to_str().expect("utf8"), Some(c), Some(t));

    assert_eq!(entries.len(), 5);
    assert_eq!(*total.lock().unwrap(), 5);
    assert_eq!(*count.lock().unwrap(), 5);
}

#[cfg(unix)]
#[test]
fn scan_directory_symlink_file_counts_as_file_not_dir() {
    use std::os::unix::fs::symlink;
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::write(root.join("real"), b"payload").expect("write");
    symlink(root.join("real"), root.join("link")).expect("symlink");

    let entries = scan_directory(root.to_str().expect("utf8"));
    let link = entries.iter().find(|e| e.name == "link").expect("link row");
    assert!(!link.is_dir);
    assert_eq!(link.size, 7);
}
