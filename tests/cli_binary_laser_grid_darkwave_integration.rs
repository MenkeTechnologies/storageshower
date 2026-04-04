//! Binary smoke: `laser-grid` and `darkwave` color flags.

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
fn laser_grid_header_version() {
    let o = output(&["--color", "laser-grid", "--header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn darkwave_no_used_help() {
    let o = output(&["--color", "darkwave", "--no-used", "--help"]);
    assert!(o.status.success());
}
