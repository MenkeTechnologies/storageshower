//! `scan_directory` with real files under a temp directory.

use std::fs::File;
use std::io::Write;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn single_file_lists_one_entry() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("hello.txt");
    let mut f = File::create(&path).expect("create");
    f.write_all(b"content").expect("write");
    drop(f);

    let p = dir.path().to_str().expect("utf8");
    let v = scan_directory(p);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].name, "hello.txt");
    assert!(!v[0].is_dir);
}

#[test]
fn two_files_sorted_by_size_descending() {
    let dir = tempdir().expect("tempdir");
    let small = dir.path().join("small.bin");
    let big = dir.path().join("big.bin");
    File::create(&small)
        .expect("create")
        .write_all(&[0u8; 10])
        .expect("write");
    File::create(&big)
        .expect("create")
        .write_all(&[1u8; 1000])
        .expect("write");

    let p = dir.path().to_str().expect("utf8");
    let v = scan_directory(p);
    assert_eq!(v.len(), 2);
    assert!(v[0].size >= v[1].size);
}
