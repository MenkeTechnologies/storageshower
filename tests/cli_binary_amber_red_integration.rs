//! Binary smoke: `amber` and `red` color flags.

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
fn amber_help() {
    let o = output(&["--color", "amber", "--help"]);
    assert!(o.status.success());
}

#[test]
fn red_version_units_bytes() {
    let o = output(&["--color", "red", "-u", "bytes", "-V"]);
    assert!(o.status.success());
}
