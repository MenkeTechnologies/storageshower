//! Binary smoke: `megacorp` and `laser-grid` color flags.

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
fn megacorp_sort_size_version() {
    let o = output(&["--color", "megacorp", "-s", "size", "-V"]);
    assert!(o.status.success());
}

#[test]
fn laser_grid_no_reverse_help() {
    let o = output(&["--color", "laser-grid", "--no-reverse", "--help"]);
    assert!(o.status.success());
}
