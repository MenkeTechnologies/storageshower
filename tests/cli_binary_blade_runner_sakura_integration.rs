//! Binary smoke: `blade-runner` and `sakura` color flags.

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
fn blade_runner_reverse_version() {
    let o = output(&["--color", "blade-runner", "-R", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sakura_col_mount_help() {
    let o = output(&["--color", "sakura", "--col-mount", "24", "--help"]);
    assert!(o.status.success());
}
