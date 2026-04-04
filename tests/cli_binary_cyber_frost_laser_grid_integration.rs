//! Binary smoke: `cyber-frost` and `laser-grid` color flags.

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
fn cyber_frost_sort_size_version() {
    let o = output(&["--color", "cyber-frost", "--sort", "size", "-V"]);
    assert!(o.status.success());
}

#[test]
fn laser_grid_refresh_help() {
    let o = output(&["--color", "laser-grid", "-r", "2", "--help"]);
    assert!(o.status.success());
}
