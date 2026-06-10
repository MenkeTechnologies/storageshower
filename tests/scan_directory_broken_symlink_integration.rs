//! `scan_directory` behavior in the presence of a dangling symlink.
//!
//! The scanner calls `std::fs::metadata(entry.path())` on every direntry,
//! which **follows** symlinks. A broken symlink returns `Err` from
//! `metadata`, which `ok()?` short-circuits — the entry is silently
//! dropped. This pins that:
//!
//! 1. The scanner does NOT panic when a directory contains a dangling
//!    symlink (regression guard for any future change that calls
//!    `unwrap()` on the metadata result).
//! 2. The dangling symlink is omitted from the returned listing — sibling
//!    real entries are still reported. A future refactor that switches
//!    to `symlink_metadata` and surfaces broken links would change this
//!    behavior; that change should land alongside an updated test, not
//!    silently slip in.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::symlink;

use tempfile::tempdir;

use storageshower::system::scan_directory;

#[test]
fn dangling_symlink_at_top_level_is_silently_skipped_not_panicked() {
    let dir = tempdir().expect("tempdir");
    // Create one real file (sibling) and one dangling symlink pointing
    // at a path that does NOT exist.
    fs::write(dir.path().join("real.txt"), b"payload").expect("write real");
    symlink(
        dir.path().join("nonexistent-target"),
        dir.path().join("dangling-link"),
    )
    .expect("symlink");

    let entries = scan_directory(dir.path().to_str().expect("utf8 path"));

    // Sibling real file MUST still appear; broken symlink is dropped.
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert!(
        names.contains(&"real.txt"),
        "real sibling must survive scan when a dangling symlink is present, got {names:?}"
    );
    assert!(
        !names.contains(&"dangling-link"),
        "dangling symlinks should be skipped under current metadata()-follows-symlinks scan path, got {names:?}"
    );
}

#[test]
fn dangling_symlink_only_directory_returns_empty_listing_no_panic() {
    // A directory containing ONLY a broken symlink: the scanner must
    // return an empty Vec rather than panic / error. Tests the
    // "all entries get filtered out by metadata()-Err" code path.
    let dir = tempdir().expect("tempdir");
    symlink(
        dir.path().join("ghost-target"),
        dir.path().join("ghost-link"),
    )
    .expect("symlink");

    let entries = scan_directory(dir.path().to_str().expect("utf8 path"));
    assert!(
        entries.is_empty(),
        "dangling-only directory must yield empty listing, got {} entries",
        entries.len()
    );
}
