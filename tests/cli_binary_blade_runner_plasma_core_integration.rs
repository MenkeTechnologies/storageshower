//! Binary smoke: `blade-runner` and `plasma-core` color flags.

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
fn plasma_core_warn_crit_help() {
    let o = output(&["--color", "plasma-core", "-w", "50", "-C", "90", "--help"]);
    assert!(o.status.success());
}
