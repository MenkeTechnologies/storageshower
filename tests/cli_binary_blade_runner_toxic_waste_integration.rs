//! Binary smoke: `blade-runner` and `toxic-waste` color flags.

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
fn blade_runner_version() {
    let o = output(&["--color", "blade-runner", "-V"]);
    assert!(o.status.success());
}

#[test]
fn toxic_waste_help() {
    let o = output(&["--color", "toxic-waste", "--help"]);
    assert!(o.status.success());
}
