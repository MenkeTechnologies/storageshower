//! Binary smoke: `amber` and `plasma-core` color flags.

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
fn amber_reverse_version() {
    let o = output(&["--color", "amber", "-R", "-V"]);
    assert!(o.status.success());
}

#[test]
fn plasma_core_no_bars_help() {
    let o = output(&["--color", "plasma-core", "--no-bars", "--help"]);
    assert!(o.status.success());
}
