//! Binary smoke: `amber` and `sunset` color flags.

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
fn amber_no_header_version() {
    let o = output(&["--color", "amber", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sunset_reverse_version() {
    let o = output(&["--color", "sunset", "-R", "-V"]);
    assert!(o.status.success());
}
