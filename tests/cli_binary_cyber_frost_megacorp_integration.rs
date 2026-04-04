//! Binary smoke: `cyber-frost` and `megacorp` color flags.

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
fn cyber_frost_local_only_version() {
    let o = output(&["--color", "cyber-frost", "-l", "-V"]);
    assert!(o.status.success());
}

#[test]
fn megacorp_reverse_version() {
    let o = output(&["--color", "megacorp", "-R", "-V"]);
    assert!(o.status.success());
}
