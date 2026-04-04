//! Binary smoke: `steel-nerve` and `zaibatsu` color flags.

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
fn steel_nerve_no_compact_version() {
    let o = output(&["--color", "steel-nerve", "--no-compact", "-V"]);
    assert!(o.status.success());
}

#[test]
fn zaibatsu_warn_crit_version() {
    let o = output(&["--color", "zaibatsu", "-w", "71", "-C", "91", "-V"]);
    assert!(o.status.success());
}
