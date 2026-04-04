//! Binary smoke: `laser-grid` and `sunset` color flags.

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
fn laser_grid_no_virtual_version() {
    let o = output(&["--color", "laser-grid", "--no-virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sunset_refresh_help() {
    let o = output(&["--color", "sunset", "-r", "5", "--help"]);
    assert!(o.status.success());
}
