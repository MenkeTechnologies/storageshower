//! Binary smoke: `laser-grid` and `glitch-pop` color flags.

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
fn laser_grid_tooltips_version() {
    let o = output(&["--color", "laser-grid", "--tooltips", "-V"]);
    assert!(o.status.success());
}

#[test]
fn glitch_pop_no_virtual_version() {
    let o = output(&["--color", "glitch-pop", "--no-virtual", "-V"]);
    assert!(o.status.success());
}
