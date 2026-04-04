//! `is_network_fs` is case-sensitive on the fstype string.

use storageshower::system::is_network_fs;

#[test]
fn lowercase_nfs_is_network() {
    assert!(is_network_fs("nfs"));
}

#[test]
fn uppercase_nfs_is_not_network() {
    assert!(!is_network_fs("NFS"));
}
