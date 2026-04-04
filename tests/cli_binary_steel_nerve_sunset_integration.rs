//! Binary smoke: `steel-nerve` and `sunset` color flags.

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
fn steel_nerve_no_header_version() {
    let o = output(&["--color", "steel-nerve", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sunset_bar_ascii_help() {
    let o = output(&["--color", "sunset", "-b", "ascii", "--help"]);
    assert!(o.status.success());
}
