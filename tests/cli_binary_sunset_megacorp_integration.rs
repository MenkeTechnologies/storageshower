//! Binary smoke: `sunset` and `megacorp` color flags.

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
fn sunset_refresh_version() {
    let o = output(&["--color", "sunset", "-r", "3", "-V"]);
    assert!(o.status.success());
}

#[test]
fn megacorp_col_pct_version() {
    let o = output(&["--color", "megacorp", "--col-pct", "14", "-V"]);
    assert!(o.status.success());
}
