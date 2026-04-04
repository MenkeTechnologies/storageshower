//! Binary smoke: `cyber-frost` and `steel-nerve` color flags.

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
fn cyber_frost_version() {
    let o = output(&["--color", "cyber-frost", "-V"]);
    assert!(o.status.success());
}

#[test]
fn steel_nerve_help() {
    let o = output(&["--color", "steel-nerve", "--help"]);
    assert!(o.status.success());
}
