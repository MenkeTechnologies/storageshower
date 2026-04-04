//! Binary smoke: `sakura` and `cyber-frost` color flags.

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
fn sakura_no_compact_version() {
    let o = output(&["--color", "sakura", "--no-compact", "-V"]);
    assert!(o.status.success());
}

#[test]
fn cyber_frost_full_mount_version() {
    let o = output(&["--color", "cyber-frost", "--full-mount", "-V"]);
    assert!(o.status.success());
}
