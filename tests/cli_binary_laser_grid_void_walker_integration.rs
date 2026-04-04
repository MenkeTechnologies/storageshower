//! Binary smoke: `laser-grid` and `void-walker` color flags.

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
fn laser_grid_full_mount_version() {
    let o = output(&["--color", "laser-grid", "-f", "-V"]);
    assert!(o.status.success());
}

#[test]
fn void_walker_units_human_help() {
    let o = output(&["--color", "void-walker", "--units", "human", "--help"]);
    assert!(o.status.success());
}
