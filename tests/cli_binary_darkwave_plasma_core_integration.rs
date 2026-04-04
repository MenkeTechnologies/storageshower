//! Binary smoke: `darkwave` and `plasma-core` color flags.

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
fn darkwave_warn_crit_version() {
    let o = output(&["--color", "darkwave", "-w", "62", "-C", "88", "-V"]);
    assert!(o.status.success());
}

#[test]
fn plasma_core_virtual_version() {
    let o = output(&["--color", "plasma-core", "--virtual", "-V"]);
    assert!(o.status.success());
}
