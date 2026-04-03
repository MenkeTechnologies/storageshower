//! `scan_directory` with nested files and directories (`dir_size` aggregation).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn subdirectory_file_contributes_to_dir_size() {
    let dir = tempdir().expect("tempdir");
    let sub = dir.path().join("nested");
    fs::create_dir_all(&sub).expect("mkdir");
    fs::write(sub.join("blob.bin"), vec![7u8; 400]).expect("write");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    let sub_ent = entries
        .iter()
        .find(|e| e.name == "nested")
        .expect("nested entry");
    assert!(sub_ent.is_dir);
    assert!(sub_ent.size >= 400, "dir size {}", sub_ent.size);
}

#[test]
fn multiple_files_in_flat_dir_sorted_by_size() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("a"), [1u8]).expect("a");
    fs::write(dir.path().join("b"), [1u8; 50]).expect("b");
    fs::write(dir.path().join("c"), [1u8; 10]).expect("c");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].name, "b");
    assert_eq!(entries[1].name, "c");
    assert_eq!(entries[2].name, "a");
}

#[test]
fn empty_subdirectory_has_zero_size() {
    let dir = tempdir().expect("tempdir");
    fs::create_dir_all(dir.path().join("emptydir")).expect("mkdir");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    let e = entries
        .iter()
        .find(|x| x.name == "emptydir")
        .expect("entry");
    assert!(e.is_dir);
    assert_eq!(e.size, 0);
}

#[test]
fn nested_empty_chain() {
    let dir = tempdir().expect("tempdir");
    fs::create_dir_all(dir.path().join("a").join("b").join("c")).expect("mkdir");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "a");
    assert!(entries[0].is_dir);
}

#[test]
fn file_and_dir_sibling_names_distinct() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("x"), b"hello").expect("file");
    fs::create_dir(dir.path().join("y")).expect("dir");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&"x"));
    assert!(names.contains(&"y"));
}
