//! Binary smoke: `megacorp` and `dark-signal` color flags.

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
fn megacorp_no_used_version() {
    let o = output(&["--color", "megacorp", "--no-used", "-V"]);
    assert!(o.status.success());
}

#[test]
fn dark_signal_refresh_help() {
    let o = output(&["--color", "dark-signal", "-r", "3", "--help"]);
    assert!(o.status.success());
}
