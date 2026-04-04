//! Binary smoke: `cyber-frost` and `blade-runner` color flags.

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
fn cyber_frost_reverse_version() {
    let o = output(&["--color", "cyber-frost", "-R", "-V"]);
    assert!(o.status.success());
}

#[test]
fn blade_runner_refresh_version() {
    let o = output(&["--color", "blade-runner", "-r", "7", "-V"]);
    assert!(o.status.success());
}
