//! Binary smoke: `blade-runner` and `megacorp` color flags.

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
fn blade_runner_tooltips_version() {
    let o = output(&["--color", "blade-runner", "--tooltips", "-V"]);
    assert!(o.status.success());
}

#[test]
fn megacorp_virtual_help() {
    let o = output(&["--color", "megacorp", "--virtual", "--help"]);
    assert!(o.status.success());
}
