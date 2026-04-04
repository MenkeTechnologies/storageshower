//! Binary smoke: `void-walker` and `steel-nerve` color flags.

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
fn void_walker_help() {
    let o = output(&["--color", "void-walker", "--help"]);
    assert!(o.status.success());
}

#[test]
fn steel_nerve_refresh_version() {
    let o = output(&["--color", "steel-nerve", "-r", "6", "-V"]);
    assert!(o.status.success());
}
