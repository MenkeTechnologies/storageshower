//! `system::is_network_fs` — every documented network FSTYPE and representative locals.

use storageshower::system::is_network_fs;

#[test]
fn detects_nfs() {
    assert!(is_network_fs("nfs"));
}

#[test]
fn detects_nfs4() {
    assert!(is_network_fs("nfs4"));
}

#[test]
fn detects_cifs() {
    assert!(is_network_fs("cifs"));
}

#[test]
fn detects_smbfs() {
    assert!(is_network_fs("smbfs"));
}

#[test]
fn detects_afp() {
    assert!(is_network_fs("afp"));
}

#[test]
fn detects_ncp() {
    assert!(is_network_fs("ncp"));
}

#[test]
fn detects_fuse_sshfs() {
    assert!(is_network_fs("fuse.sshfs"));
}

#[test]
fn detects_fuse_rclone() {
    assert!(is_network_fs("fuse.rclone"));
}

#[test]
fn detects_fuse_s3fs() {
    assert!(is_network_fs("fuse.s3fs"));
}

#[test]
fn detects_9p() {
    assert!(is_network_fs("9p"));
}

#[test]
fn detects_afs() {
    assert!(is_network_fs("afs"));
}

#[test]
fn rejects_ext4() {
    assert!(!is_network_fs("ext4"));
}

#[test]
fn rejects_apfs() {
    assert!(!is_network_fs("apfs"));
}

#[test]
fn rejects_xfs_btrfs_tmpfs() {
    assert!(!is_network_fs("xfs"));
    assert!(!is_network_fs("btrfs"));
    assert!(!is_network_fs("tmpfs"));
}

#[test]
fn rejects_empty_string() {
    assert!(!is_network_fs(""));
}

#[test]
fn rejects_substring_not_whole_type() {
    assert!(!is_network_fs("nfs4backup"));
}
