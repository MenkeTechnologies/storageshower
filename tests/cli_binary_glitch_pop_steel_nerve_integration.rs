//! Binary smoke: `glitch-pop` and `steel-nerve` color flags.

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
fn glitch_pop_version() {
    let o = output(&["--color", "glitch-pop", "-V"]);
    assert!(o.status.success());
}

#[test]
fn steel_nerve_sort_size_help() {
    let o = output(&["--color", "steel-nerve", "--sort", "size", "--help"]);
    assert!(o.status.success());
}
