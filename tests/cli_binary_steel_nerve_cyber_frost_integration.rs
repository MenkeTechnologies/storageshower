//! Binary smoke: `steel-nerve` and `cyber-frost` color flags.

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
fn steel_nerve_border_version() {
    let o = output(&["--color", "steel-nerve", "--border", "-V"]);
    assert!(o.status.success());
}

#[test]
fn cyber_frost_sort_size_help() {
    let o = output(&["--color", "cyber-frost", "--sort", "size", "--help"]);
    assert!(o.status.success());
}
