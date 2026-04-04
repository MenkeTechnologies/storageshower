//! `scan_directory` with two hundred seventeen files (largest first).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn two_hundred_seventeen_files_order_by_size_desc() {
    let dir = tempdir().expect("tempdir");
    for i in 0usize..217 {
        let sz = (i + 1) * 7;
        fs::write(dir.path().join(format!("n{i}")), vec![0u8; sz]).expect("w");
    }
    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 217);
    assert_eq!(v[0].name, "n216");
    assert!(v[0].size >= v[216].size);
}
