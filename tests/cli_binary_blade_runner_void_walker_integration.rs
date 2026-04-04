//! Binary smoke: `blade-runner` and `void-walker` color flags.

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
fn blade_runner_refresh_version() {
    let o = output(&["--color", "blade-runner", "-r", "8", "-V"]);
    assert!(o.status.success());
}

#[test]
fn void_walker_no_header_version() {
    let o = output(&["--color", "void-walker", "--no-header", "-V"]);
    assert!(o.status.success());
}
