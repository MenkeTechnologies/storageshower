//! Binary smoke: `night-city` and `laser-grid` color flags.

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
fn night_city_units_bytes_version() {
    let o = output(&["--color", "night-city", "-u", "bytes", "-V"]);
    assert!(o.status.success());
}

#[test]
fn laser_grid_no_header_version() {
    let o = output(&["--color", "laser-grid", "--no-header", "-V"]);
    assert!(o.status.success());
}
