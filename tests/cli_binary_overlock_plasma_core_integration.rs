//! Binary smoke: `overlock` and `plasma-core` color flags.

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
fn overlock_help() {
    let o = output(&["--color", "overlock", "--help"]);
    assert!(o.status.success());
}

#[test]
fn plasma_core_version_reverse() {
    let o = output(&["--color", "plasma-core", "-R", "-V"]);
    assert!(o.status.success());
}
