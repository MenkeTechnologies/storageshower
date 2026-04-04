//! `scan_directory` with two hundred forty-six files (largest first).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn two_hundred_forty_six_files_order_by_size_desc() {
    let dir = tempdir().expect("tempdir");
    for i in 0usize..246 {
        let sz = (i + 1) * 7;
        fs::write(dir.path().join(format!("n{i}")), vec![0u8; sz]).expect("w");
    }
    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 246);
    assert_eq!(v[0].name, "n245");
    assert!(v[0].size >= v[245].size);
}
