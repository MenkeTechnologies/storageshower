//! Binary smoke: `neon-noir` and `holo-shift` color flags.

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
fn neon_noir_local_only_version() {
    let o = output(&["--color", "neon-noir", "--local-only", "-V"]);
    assert!(o.status.success());
}

#[test]
fn holo_shift_full_mount_help() {
    let o = output(&["--color", "holo-shift", "--full-mount", "--help"]);
    assert!(o.status.success());
}
