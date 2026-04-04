//! Binary smoke: `default` and `zaibatsu` color flags.

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
fn default_local_only_version() {
    let o = output(&["--color", "default", "-l", "-V"]);
    assert!(o.status.success());
}

#[test]
fn zaibatsu_no_header_help() {
    let o = output(&["--color", "zaibatsu", "--no-header", "--help"]);
    assert!(o.status.success());
}
