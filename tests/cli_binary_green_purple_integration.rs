//! Binary smoke: `green` and `purple` color flags.

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
fn green_local_only_version() {
    let o = output(&["--color", "green", "-l", "-V"]);
    assert!(o.status.success());
}

#[test]
fn purple_no_virtual_version() {
    let o = output(&["--color", "purple", "--no-virtual", "-V"]);
    assert!(o.status.success());
}
