//! `scan_directory_with_progress` when only one of the progress mutexes is supplied.

use std::fs;
use std::sync::{Arc, Mutex};

use tempfile::tempdir;

use storageshower::system::scan_directory_with_progress;

#[test]
fn only_count_mutex_updates_progress() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("a"), b"x").expect("a");
    fs::write(dir.path().join("b"), b"yy").expect("b");
    let count = Arc::new(Mutex::new(0usize));
    let entries = scan_directory_with_progress(
        dir.path().to_str().expect("utf8"),
        Some(count.clone()),
        None,
    );
    assert_eq!(entries.len(), 2);
    assert_eq!(*count.lock().expect("lock"), 2);
}

#[test]
fn only_total_mutex_sets_entry_count() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("only"), b"z").expect("w");
    let total = Arc::new(Mutex::new(0usize));
    let entries = scan_directory_with_progress(
        dir.path().to_str().expect("utf8"),
        None,
        Some(total.clone()),
    );
    assert_eq!(entries.len(), 1);
    assert_eq!(*total.lock().expect("lock"), 1);
}

#[test]
fn count_mutex_tracks_three_files() {
    let dir = tempdir().expect("tempdir");
    for i in 0..3 {
        fs::write(dir.path().join(format!("f{i}.txt")), b"1").expect("w");
    }
    let count = Arc::new(Mutex::new(0usize));
    let _ = scan_directory_with_progress(
        dir.path().to_str().expect("utf8"),
        Some(count.clone()),
        None,
    );
    assert_eq!(*count.lock().expect("lock"), 3);
}
