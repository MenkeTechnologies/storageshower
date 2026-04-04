//! `scan_directory_with_progress` with both `count` and `total` mutexes set.

use std::fs;
use std::sync::{Arc, Mutex};

use tempfile::tempdir;

use storageshower::system::scan_directory_with_progress;

#[test]
fn both_mutexes_updated_for_two_files() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("x"), b"a").expect("x");
    fs::write(dir.path().join("y"), b"bb").expect("y");
    let count = Arc::new(Mutex::new(0usize));
    let total = Arc::new(Mutex::new(0usize));
    let path = dir.path().to_str().expect("utf8");
    let entries = scan_directory_with_progress(path, Some(count.clone()), Some(total.clone()));
    assert_eq!(entries.len(), 2);
    assert_eq!(*count.lock().expect("lock"), 2);
    assert_eq!(*total.lock().expect("lock"), 2);
}

#[test]
fn both_mutexes_empty_dir() {
    let dir = tempdir().expect("tempdir");
    let count = Arc::new(Mutex::new(0usize));
    let total = Arc::new(Mutex::new(0usize));
    let path = dir.path().to_str().expect("utf8");
    let entries = scan_directory_with_progress(path, Some(count.clone()), Some(total.clone()));
    assert!(entries.is_empty());
    assert_eq!(*count.lock().expect("lock"), 0);
    assert_eq!(*total.lock().expect("lock"), 0);
}
