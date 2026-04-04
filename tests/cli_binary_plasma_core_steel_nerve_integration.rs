//! Binary smoke: `plasma-core` and `steel-nerve` color flags.

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
fn plasma_core_sort_size_version() {
    let o = output(&["--color", "plasma-core", "--sort", "size", "-V"]);
    assert!(o.status.success());
}

#[test]
fn steel_nerve_no_border_help() {
    let o = output(&["--color", "steel-nerve", "--no-border", "--help"]);
    assert!(o.status.success());
}
