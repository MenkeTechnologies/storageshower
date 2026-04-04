//! Binary smoke: `blade-runner` and `zaibatsu` color flags.

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
fn blade_runner_compact_version() {
    let o = output(&["--color", "blade-runner", "-k", "-V"]);
    assert!(o.status.success());
}

#[test]
fn zaibatsu_list_colors() {
    let o = output(&["--color", "zaibatsu", "--list-colors"]);
    assert!(o.status.success());
}
