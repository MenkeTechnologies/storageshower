//! `scan_directory_with_progress`: mutex counters and error paths.

use std::fs;
use std::sync::{Arc, Mutex};

use tempfile::tempdir;

use storageshower::system::scan_directory_with_progress;

#[test]
fn progress_count_matches_total_after_scan() {
    let dir = tempdir().expect("tempdir");
    for i in 0..5 {
        let name = format!("file_{i}.txt");
        fs::write(dir.path().join(&name), vec![i as u8; i + 1]).expect("write");
    }
    let count = Arc::new(Mutex::new(0usize));
    let total = Arc::new(Mutex::new(0usize));
    let entries = scan_directory_with_progress(
        dir.path().to_str().expect("utf8"),
        Some(count.clone()),
        Some(total.clone()),
    );
    assert_eq!(entries.len(), 5);
    let c = *count.lock().expect("lock");
    let t = *total.lock().expect("lock");
    assert_eq!(c, t);
    assert_eq!(t, 5);
}

#[test]
fn nonexistent_path_returns_empty_without_panicking() {
    let count = Arc::new(Mutex::new(0usize));
    let total = Arc::new(Mutex::new(0usize));
    let entries = scan_directory_with_progress(
        "/nonexistent/storageshower_scan_xyz_999",
        Some(count.clone()),
        Some(total.clone()),
    );
    assert!(entries.is_empty());
}

#[test]
fn none_progress_callbacks_still_returns_entries() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("solo.txt"), b"x").expect("write");
    let entries = scan_directory_with_progress(dir.path().to_str().expect("utf8"), None, None);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "solo.txt");
}
