//! `scan_directory` with seven files (size-descending order).

use std::fs;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn seven_files_largest_first() {
    let dir = tempdir().expect("tempdir");
    for i in 0..7u8 {
        let sz = (i as usize + 1) * 11;
        fs::write(dir.path().join(format!("file{i}")), vec![0u8; sz]).expect("w");
    }
    let v = scan_directory(dir.path().to_str().expect("utf8"));
    assert_eq!(v.len(), 7);
    assert_eq!(v[0].name, "file6");
    assert!(v[0].size >= v[6].size);
}
