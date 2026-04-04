//! Binary smoke: `blade-runner` and `neon-noir` color flags.

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
fn blade_runner_virtual_version() {
    let o = output(&["--color", "blade-runner", "--virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn neon_noir_no_header_help() {
    let o = output(&["--color", "neon-noir", "--no-header", "--help"]);
    assert!(o.status.success());
}
