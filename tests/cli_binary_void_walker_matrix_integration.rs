//! Binary smoke: `void-walker` and `matrix` color flags.

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
fn void_walker_virtual_version() {
    let o = output(&["--color", "void-walker", "--virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn matrix_sort_pct_help() {
    let o = output(&["--color", "matrix", "-s", "pct", "--help"]);
    assert!(o.status.success());
}
