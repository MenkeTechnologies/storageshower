//! `scan_directory` includes dotfiles (hidden-style names).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn dotfile_appears_in_listing() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join(".hidden"), b"secret").expect("write");
    fs::write(dir.path().join("visible"), b"x").expect("write");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&".hidden"));
    assert!(names.contains(&"visible"));
}

#[test]
fn dot_prefixed_dir_has_computed_size() {
    let dir = tempdir().expect("tempdir");
    let sub = dir.path().join(".cache");
    fs::create_dir_all(&sub).expect("mkdir");
    fs::write(sub.join("blob"), vec![0u8; 500]).expect("write");
    let entries = scan_directory(dir.path().to_str().expect("utf8"));
    let e = entries
        .iter()
        .find(|x| x.name == ".cache")
        .expect("dot dir");
    assert!(e.is_dir);
    assert!(e.size >= 500);
}
