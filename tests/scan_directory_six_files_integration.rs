//! `scan_directory` with six files (largest first after sort).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn six_files_order_by_size_desc() {
    let dir = tempdir().expect("tempdir");
    let sizes = [1usize, 3, 9, 27, 81, 5000];
    for (i, sz) in sizes.iter().enumerate() {
        fs::write(dir.path().join(format!("n{i}")), vec![0u8; *sz]).expect("write");
    }
    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 6);
    assert_eq!(v[0].name, "n5");
    assert!(v[0].size >= v[5].size);
}
