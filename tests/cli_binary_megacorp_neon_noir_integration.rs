//! Binary smoke: `megacorp` and `neon-noir` color flags.

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
fn megacorp_help() {
    let o = output(&["--color", "megacorp", "--help"]);
    assert!(o.status.success());
}

#[test]
fn neon_noir_version_units_bytes() {
    let o = output(&["--color", "neon-noir", "-u", "bytes", "-V"]);
    assert!(o.status.success());
}
