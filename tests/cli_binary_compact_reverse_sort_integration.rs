//! Binary smoke: compact, reverse, and sort flags combined with safe exits.

use std::process::Command;

fn exe() -> &'static str {
    env!("CARGO_BIN_EXE_storageshower")
}

fn output(args: &[&str]) -> std::process::Output {
    Command::new(exe())
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", exe()))
}

#[test]
fn compact_reverse_sort_size_version() {
    let o = output(&["-k", "-R", "--sort", "size", "-V"]);
    assert!(o.status.success());
}

#[test]
fn full_mount_local_only_help() {
    let o = output(&["-f", "-l", "--help"]);
    assert!(o.status.success());
}

#[test]
fn compact_no_header_list_colors() {
    let o = output(&["-k", "--no-header", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn reverse_pct_units_gib_help() {
    let o = output(&["-R", "--sort", "pct", "--units", "gib", "--help"]);
    assert!(o.status.success());
}
