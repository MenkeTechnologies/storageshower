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
fn laser_grid_full_mount_version() {
    let o = output(&["--color", "laser-grid", "--full-mount", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sunset_no_used_version() {
    let o = output(&["--color", "sunset", "--no-used", "-V"]);
    assert!(o.status.success());
}
