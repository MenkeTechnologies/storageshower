//! Binary smoke: `laser-grid` and `steel-nerve` color flags.

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
fn laser_grid_no_used_version() {
    let o = output(&["--color", "laser-grid", "--no-used", "-V"]);
    assert!(o.status.success());
}

#[test]
fn steel_nerve_bars_version() {
    let o = output(&["--color", "steel-nerve", "--bars", "-V"]);
    assert!(o.status.success());
}
