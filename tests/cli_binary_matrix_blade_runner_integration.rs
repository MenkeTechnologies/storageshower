//! Binary smoke: `matrix` and `blade-runner` color flags.

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
fn matrix_no_border_version() {
    let o = output(&["--color", "matrix", "--no-border", "-V"]);
    assert!(o.status.success());
}

#[test]
fn blade_runner_units_mib_version() {
    let o = output(&["--color", "blade-runner", "-u", "mib", "-V"]);
    assert!(o.status.success());
}
