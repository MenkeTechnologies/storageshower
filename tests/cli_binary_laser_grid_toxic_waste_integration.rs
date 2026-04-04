//! Binary smoke: `laser-grid` and `toxic-waste` color flags.

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
fn laser_grid_border_version() {
    let o = output(&["--color", "laser-grid", "--border", "-V"]);
    assert!(o.status.success());
}

#[test]
fn toxic_waste_list_colors_help() {
    let o = output(&["--color", "toxic-waste", "--list-colors", "--help"]);
    assert!(o.status.success());
}
