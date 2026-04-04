//! Binary smoke: `overlock` and `zaibatsu` color flags.

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
fn overlock_version() {
    let o = output(&["--color", "overlock", "-V"]);
    assert!(o.status.success());
}

#[test]
fn zaibatsu_compact_help() {
    let o = output(&["--color", "zaibatsu", "-k", "--help"]);
    assert!(o.status.success());
}
