//! Binary smoke: `darkwave` and `neon-noir` color flags.

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
fn darkwave_local_only_version() {
    let o = output(&["--color", "darkwave", "-l", "-V"]);
    assert!(o.status.success());
}

#[test]
fn neon_noir_no_compact_help() {
    let o = output(&["--color", "neon-noir", "--no-compact", "--help"]);
    assert!(o.status.success());
}
