//! Binary smoke: `cyber-frost` and `darkwave` color flags.

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
fn cyber_frost_no_virtual_version() {
    let o = output(&["--color", "cyber-frost", "--no-virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn darkwave_col_pct_help() {
    let o = output(&["--color", "darkwave", "--col-pct", "10", "--help"]);
    assert!(o.status.success());
}
