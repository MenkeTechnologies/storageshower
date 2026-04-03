//! Extended `truncate_mount` coverage via the public `helpers` API.

use storageshower::helpers::truncate_mount;

#[test]
fn truncate_width_1_ascii() {
    let r = truncate_mount("/ab", 1);
    assert_eq!(r.chars().count(), 1);
}

#[test]
fn truncate_width_2_ascii() {
    let r = truncate_mount("/ab", 2);
    assert_eq!(r.chars().count(), 2);
}

#[test]
fn truncate_exact_len_no_ellipsis() {
    let s = "/mnt/x";
    assert_eq!(truncate_mount(s, s.chars().count()), s);
}

#[test]
fn truncate_one_char_wider_than_mount_pads() {
    let s = "/a";
    let target_w = s.chars().count() + 1;
    let r = truncate_mount(s, target_w);
    assert_eq!(r.chars().count(), target_w);
}

#[test]
fn truncate_cjk_narrow_width() {
    let r = truncate_mount("/日本", 2);
    assert!(r.chars().count() <= 2);
}

#[test]
fn truncate_mixed_ascii_cjk() {
    let r = truncate_mount("/mnt/語", 6);
    assert!(r.chars().count() <= 6);
}

#[test]
fn truncate_very_long_mount_20_cols() {
    let long = "/".to_string() + &"segment/".repeat(30);
    let r = truncate_mount(&long, 20);
    assert_eq!(r.chars().count(), 20);
    assert!(r.ends_with('\u{2026}'));
}

#[test]
fn truncate_preserves_start_when_wide_enough() {
    let r = truncate_mount("/very/long/path", 14);
    assert!(r.starts_with('/'));
}

#[test]
fn truncate_single_char_mount_width_10() {
    let r = truncate_mount("x", 10);
    assert_eq!(r.chars().count(), 10);
}

#[test]
fn truncate_spaces_in_mount() {
    let r = truncate_mount("/Volumes/My Disk", 8);
    assert_eq!(r.chars().count(), 8);
}
