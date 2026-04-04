//! Binary smoke: `sunset` and `darkwave` color flags.

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
fn sunset_version() {
    let o = output(&["--color", "sunset", "-V"]);
    assert!(o.status.success());
}

#[test]
fn darkwave_no_border_help() {
    let o = output(&["--color", "darkwave", "--no-border", "--help"]);
    assert!(o.status.success());
}
