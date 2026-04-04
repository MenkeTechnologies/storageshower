//! Binary smoke: `sakura` and `overlock` color flags.

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
fn sakura_full_mount_version() {
    let o = output(&["--color", "sakura", "--full-mount", "-V"]);
    assert!(o.status.success());
}

#[test]
fn overlock_no_used_help() {
    let o = output(&["--color", "overlock", "--no-used", "--help"]);
    assert!(o.status.success());
}
