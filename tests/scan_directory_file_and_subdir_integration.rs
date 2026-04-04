//! `scan_directory` with a file and a subdirectory (subdir size includes nested file).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn two_top_level_entries_file_and_dir() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("top.txt"), [0u8; 3]).expect("top");
    let sub = dir.path().join("nested");
    fs::create_dir(&sub).expect("mkdir");
    fs::write(sub.join("inner.bin"), [0u8; 200]).expect("inner");

    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 2);
    let names: Vec<_> = v.iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&"top.txt"));
    assert!(names.contains(&"nested"));
}

#[test]
fn larger_directory_sorts_before_smaller_file() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("tiny"), [0u8; 1]).expect("tiny");
    let sub = dir.path().join("bigdir");
    fs::create_dir(&sub).expect("mkdir");
    fs::write(sub.join("blob"), [0u8; 5000]).expect("blob");

    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].name, "bigdir");
    assert!(v[0].size >= v[1].size);
}
