//! Binary smoke: `neon-noir` and `laser-grid` color flags.

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
fn neon_noir_virtual_version() {
    let o = output(&["--color", "neon-noir", "--virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn laser_grid_no_compact_help() {
    let o = output(&["--color", "laser-grid", "--no-compact", "--help"]);
    assert!(o.status.success());
}
