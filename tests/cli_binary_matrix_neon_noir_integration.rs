//! Binary smoke: `matrix` and `neon-noir` color flags.

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
fn matrix_virtual_version() {
    let o = output(&["--color", "matrix", "--virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn neon_noir_no_compact_version() {
    let o = output(&["--color", "neon-noir", "--no-compact", "-V"]);
    assert!(o.status.success());
}
