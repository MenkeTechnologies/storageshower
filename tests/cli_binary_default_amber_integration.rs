//! Binary smoke: `default` and `amber` color flags.

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
fn default_compact_version() {
    let o = output(&["--color", "default", "--compact", "-V"]);
    assert!(o.status.success());
}

#[test]
fn amber_warn_crit_version() {
    let o = output(&["--color", "amber", "-w", "64", "-C", "89", "-V"]);
    assert!(o.status.success());
}
