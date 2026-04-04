//! Binary smoke: `steel-nerve` and `matrix` color flags.

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
fn steel_nerve_header_version() {
    let o = output(&["--color", "steel-nerve", "--header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn matrix_no_virtual_help() {
    let o = output(&["--color", "matrix", "--no-virtual", "--help"]);
    assert!(o.status.success());
}
