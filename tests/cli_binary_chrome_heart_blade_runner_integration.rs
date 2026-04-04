//! Binary smoke: `chrome-heart` and `blade-runner` color flags.

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
fn chrome_heart_local_only_version() {
    let o = output(&["--color", "chrome-heart", "-l", "-V"]);
    assert!(o.status.success());
}

#[test]
fn blade_runner_no_virtual_version() {
    let o = output(&["--color", "blade-runner", "--no-virtual", "-V"]);
    assert!(o.status.success());
}
