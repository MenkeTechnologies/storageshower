//! `scan_directory` with sixteen files (largest first).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn sixteen_files_order_by_size_desc() {
    let dir = tempdir().expect("tempdir");
    for i in 0..16u8 {
        let sz = (i as usize + 1) * 7;
        fs::write(dir.path().join(format!("n{i}")), vec![0u8; sz]).expect("w");
    }
    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 16);
    assert_eq!(v[0].name, "n15");
    assert!(v[0].size >= v[15].size);
}
