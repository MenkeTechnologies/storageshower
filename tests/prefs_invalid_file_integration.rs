//! `load_prefs_from(Some(path))` when the file is missing or not valid TOML.

use std::fs;

use tempfile::tempdir;

use storageshower::prefs::load_prefs_from;
use storageshower::types::SortMode;

#[test]
fn invalid_toml_returns_defaults() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("bad.toml");
    fs::write(&path, "this is not [[valid]] toml {{{").expect("write");
    let p = load_prefs_from(Some(path.to_str().expect("utf8")));
    assert_eq!(p.sort_mode, SortMode::Name);
    assert_eq!(p.refresh_rate, 1);
}

#[test]
fn missing_path_returns_defaults() {
    let p = load_prefs_from(Some("/tmp/does-not-exist-storageshower-prefs-99999.conf"));
    assert_eq!(p.sort_mode, SortMode::Name);
}
